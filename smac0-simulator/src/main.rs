fn main() {
    let memory: [usize; 1000] = [0; 1000];
    let registers: [usize; 4] = [0; 4];
    let program_counter: usize = 0;
    let condition_codes: [bool; 6] = [false; 6];
    let last_logical_addr: usize = 0;

    smac0_simulator::build(memory, registers, program_counter, condition_codes, last_logical_addr);
}
