module A::mod1 {
    public struct S { f: u64 }
    public fun t(mut x: u64, mut yip: u64, s: S): u64  {
        let mut yes = 0;
        let S { f: mut fin } = s;
        // these four assignments necessitate mut annotations above
        yip = 0;
        x = yes + 1;
        fin = fin + 1;
        yes = x + fin;
        yes
    }
}
