#[inline]
#[allow(dead_code)]
fn readgsbase<T>() -> *const T {
    unsafe {
        let x;
        std::arch::asm!(
            "rdgsbase {x}",
            x = out(reg) x,
            //options(pure, nomem),
        );
        x
    }
}


macro_rules! memory_accessors {
    ($ty:ty, $read:ident, $write:ident) => {
        #[inline]
        #[allow(dead_code, unused_variables)]
        fn $read(memory: &[u8], addr: usize) -> Option<$ty> {
            Some(<$ty>::from_le_bytes(
                <<MEMORYGET>>
                    .try_into()
                    .ok()?,
            ))
        }

        #[inline]
        #[allow(dead_code, unused_variables)]
        fn $write(memory: &mut [u8], addr: usize, value: $ty) -> Option<()> {
            <<MEMORYGETMUT>>
                .copy_from_slice(&value.to_le_bytes());
            Some(())
        }
    };
}

macro_rules! memory_accessors_x86 {
    ($ty:ty, $read:ident, $write:ident) => {
        #[inline]
        #[allow(dead_code, unused_variables)]
        fn $read(addr: usize) -> Option<$ty> {
            unsafe{
                let gs: *const $ty = readgsbase();
                let read_loc: *const $ty = gs.add(addr);
                Some(std::ptr::read(read_loc))
            }
            // Some(<$ty>::from_le_bytes(
            //     <<MEMORYGET>>
            //         .try_into()
            //         .ok()?,
            // ))
        }

        #[inline]
        #[allow(dead_code, unused_variables)]
        fn $write(addr: usize, value: $ty) -> Option<()> {
            unsafe{
                let gs: *const $ty = readgsbase();
                let read_loc: *mut $ty = gs.add(addr) as *mut $ty;
                std::ptr::write(read_loc, value);
            }
            Some(())
        }
    };
}
// #[inline]
// #[allow(dead_code, unused_variables)]
// fn read_mem_x86<T: From<TaggedVal>>(addr: usize) -> T {
//     unsafe{
//         let gs = readgsbase();
//         let read_loc = gs.add(addr);
//         ptr::read(read_loc)
//     }
// }

// #[inline]
// #[allow(dead_code, unused_variables)]
// fn write_mem_x86<T>(addr: usize, value: T) {
//     unsafe{
//         let gs = readgsbase();
//         let read_loc = gs.add(addr);
//         ptr::write(read_loc, value);
//     };
// }


memory_accessors! {u8, read_mem_u8, write_mem_u8}
memory_accessors! {u16, read_mem_u16, write_mem_u16}
memory_accessors! {u32, read_mem_u32, write_mem_u32}
memory_accessors! {u64, read_mem_u64, write_mem_u64}

memory_accessors! {i8, read_mem_i8, write_mem_i8}
memory_accessors! {i16, read_mem_i16, write_mem_i16}
memory_accessors! {i32, read_mem_i32, write_mem_i32}
memory_accessors! {i64, read_mem_i64, write_mem_i64}

memory_accessors! {f32, read_mem_f32, write_mem_f32}
memory_accessors! {f64, read_mem_f64, write_mem_f64}


// x86 readers/writers
memory_accessors_x86! {u8, x86_read_mem_u8, x86_write_mem_u8}
memory_accessors_x86! {u16, x86_read_mem_u16, x86_write_mem_u16}
memory_accessors_x86! {u32, x86_read_mem_u32, x86_write_mem_u32}
memory_accessors_x86! {u64, x86_read_mem_u64, x86_write_mem_u64}

memory_accessors_x86! {i8, x86_read_mem_i8, x86_write_mem_i8}
memory_accessors_x86! {i16, x86_read_mem_i16, x86_write_mem_i16}
memory_accessors_x86! {i32, x86_read_mem_i32, x86_write_mem_i32}
memory_accessors_x86! {i64, x86_read_mem_i64, x86_write_mem_i64}

memory_accessors_x86! {f32, x86_read_mem_f32, x86_write_mem_f32}
memory_accessors_x86! {f64, x86_read_mem_f64, x86_write_mem_f64}