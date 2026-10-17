//! SDK-compatible IP Helper ABI; required feature gates are deliberately not
//! added to the locked manifest. Layout and offsets match MIB_IF_ROW2/TABLE2.
use super::*;
use std::{ffi::c_void, ptr::NonNull};
use windows_sys::core::GUID;

#[repr(C)]
struct Row {
    luid: u64,
    index: u32,
    guid: GUID,
    alias: [u16; 257],
    description: [u16; 257],
    physical_length: u32,
    physical: [u8; 32],
    permanent: [u8; 32],
    mtu: u32,
    kind: u32,
    tunnel: i32,
    media: i32,
    physical_medium: i32,
    access: i32,
    direction: i32,
    flags: u8,
    operational: i32,
    admin: i32,
    connected: i32,
    network_guid: GUID,
    connection: i32,
    transmit_speed: u64,
    receive_speed: u64,
    // InOctets..OutQLen, eighteen consecutive ULONG64 members in SDK order.
    counters: [u64; 18],
}
#[repr(C)]
struct Table {
    count: u32,
    rows: [Row; 1],
}
#[link(name = "iphlpapi")]
unsafe extern "system" {
    fn GetIfTable2(table: *mut *mut Table) -> u32;
    fn FreeMibTable(table: *const c_void);
}
struct OwnedTable(NonNull<Table>);
impl Drop for OwnedTable {
    fn drop(&mut self) {
        // SAFETY: Exact allocation returned by successful GetIfTable2, freed
        // once after all row borrows/copies have ended; never crosses a thread.
        unsafe { FreeMibTable(self.0.as_ptr().cast()) };
    }
}
pub(super) fn sample() -> Result<Vec<Interface>, NativeError> {
    let mut pointer = std::ptr::null_mut();
    // SAFETY: Valid out-pointer; the OS allocates the entire variable table.
    let code = unsafe { GetIfTable2(&mut pointer) };
    if code != 0 {
        return Err(NativeError {
            operation: "GetIfTable2",
            code,
        });
    }
    let table = OwnedTable(NonNull::new(pointer).ok_or(NativeError {
        operation: "GetIfTable2 null table",
        code: 13,
    })?);
    // SAFETY: Successful API allocation includes this header, even at count 0.
    let count = unsafe { (*table.0.as_ptr()).count } as usize;
    if count > MAX_INTERFACES {
        return Err(NativeError {
            operation: "GetIfTable2 interface limit",
            code: 234,
        });
    }
    // SAFETY: addr_of does not form a reference to the nominal one-row array.
    // The ABI Table offset preserves the SDK's padding before its first row.
    let first = unsafe { std::ptr::addr_of!((*table.0.as_ptr()).rows).cast::<Row>() };
    let mut rows = Vec::with_capacity(count);
    for index in 0..count {
        // SAFETY: GetIfTable2 guarantees count complete SDK rows. The repr(C)
        // layout includes inter-row alignment; index is strictly below count.
        let row = unsafe { &*first.add(index) };
        let guid = (u128::from(row.guid.data1) << 96)
            | (u128::from(row.guid.data2) << 80)
            | (u128::from(row.guid.data3) << 64)
            | u128::from(u64::from_be_bytes(row.guid.data4));
        rows.push(Interface {
            id: Identity {
                guid,
                luid: row.luid,
            },
            kind: row.kind,
            flags: row.flags,
            operational: row.operational == 1,
            admin_up: row.admin == 1,
            connected: row.connected == 1,
            received: row.counters[0],
            transmitted: row.counters[9],
        });
    }
    Ok(rows)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sdk_layout_and_real_table_are_valid() {
        assert_eq!(size_of::<Row>(), 1352);
        assert_eq!(std::mem::offset_of!(Table, rows), 8);
        assert_eq!(std::mem::offset_of!(Row, operational), 1156);
        assert_eq!(std::mem::offset_of!(Row, counters), 1208);
        let rows = sample().expect("Read-only local IP Helper table");
        assert!(!rows.is_empty());
        assert!(rows.iter().all(|row| row.id.guid != 0 && row.id.luid != 0));
    }
}
