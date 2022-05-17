use x86_64::{
    VirtAddr, PhysAddr,
    structures::paging::{
        PageTable, OffsetPageTable,
        PhysFrame, Size4KiB, FrameAllocator,
    },
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = unsafe { active_level_4_table(physical_memory_offset) };
    unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) }
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        Self { memory_map, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Memory Regions
        self.memory_map.iter()
            // Usable memory regions
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            // Address ranges
            .map(|r| r.range.start_addr()..r.range.end_addr())
            // Frame addresses
            .flat_map(|r| r.step_by(4096))
            // Physical frames
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
