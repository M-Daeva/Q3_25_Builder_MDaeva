pub fn get_space(struct_space: usize) -> usize {
    const DISCRIMINATOR_SPACE: usize = 8;

    DISCRIMINATOR_SPACE + struct_space
}
