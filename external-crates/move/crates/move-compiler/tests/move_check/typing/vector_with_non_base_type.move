module 0x42::Test {
    fun t() {
        // test invalid vector instantiation
        let _ = vector<&u64>[];
        let _ = vector<&mut u64>[];
        let _ = vector<()>[];
        let _ = vector<(u64, bool)>[];
    }
}
