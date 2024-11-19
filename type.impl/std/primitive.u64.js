(function() {var type_impls = {
"consensus_config":[],
"iota_core":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RandPrime%3Cu64%3E-for-R\" class=\"impl\"><a href=\"#impl-RandPrime%3Cu64%3E-for-R\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; RandPrime&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a>&gt; for R<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand/rng/trait.Rng.html\" title=\"trait rand::rng::Rng\">Rng</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime\" class=\"method trait-impl\"><a href=\"#method.gen_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>, _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime within the given bit size limit <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime_exact</a>(\n    &amp;mut self,\n    bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>,\n    _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;,\n) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime with <strong>exact</strong> the given bit size <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime within the given bit size limit. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime_exact</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime with the <strong>exact</strong> given bit size. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details></div></details>","RandPrime<u64>","iota_core::checkpoints::CheckpointHeight","iota_core::epoch::randomness::CommitTimestampMs"]],
"iota_genesis_builder":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RandPrime%3Cu64%3E-for-R\" class=\"impl\"><a href=\"#impl-RandPrime%3Cu64%3E-for-R\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; RandPrime&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a>&gt; for R<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand/rng/trait.Rng.html\" title=\"trait rand::rng::Rng\">Rng</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime\" class=\"method trait-impl\"><a href=\"#method.gen_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>, _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime within the given bit size limit <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime_exact</a>(\n    &amp;mut self,\n    bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>,\n    _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;,\n) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime with <strong>exact</strong> the given bit size <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime within the given bit size limit. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime_exact</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime with the <strong>exact</strong> given bit size. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details></div></details>","RandPrime<u64>","iota_genesis_builder::stardust::migration::migration::ExpirationTimestamp"]],
"iota_rosetta":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RandPrime%3Cu64%3E-for-R\" class=\"impl\"><a href=\"#impl-RandPrime%3Cu64%3E-for-R\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; RandPrime&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a>&gt; for R<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand/rng/trait.Rng.html\" title=\"trait rand::rng::Rng\">Rng</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime\" class=\"method trait-impl\"><a href=\"#method.gen_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>, _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime within the given bit size limit <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime_exact</a>(\n    &amp;mut self,\n    bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>,\n    _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;,\n) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime with <strong>exact</strong> the given bit size <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime within the given bit size limit. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime_exact</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime with the <strong>exact</strong> given bit size. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details></div></details>","RandPrime<u64>","iota_rosetta::types::BlockHeight"]],
"iota_types":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CSequenceNumber%3E-for-u64\" class=\"impl\"><a class=\"src rightside\" href=\"src/iota_types/base_types.rs.html#1141-1145\">source</a><a href=\"#impl-From%3CSequenceNumber%3E-for-u64\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"iota_types/base_types/struct.SequenceNumber.html\" title=\"struct iota_types::base_types::SequenceNumber\">SequenceNumber</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/iota_types/base_types.rs.html#1142-1144\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(val: <a class=\"struct\" href=\"iota_types/base_types/struct.SequenceNumber.html\" title=\"struct iota_types::base_types::SequenceNumber\">SequenceNumber</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<SequenceNumber>","iota_types::base_types::TxSequenceNumber","iota_types::base_types::CommitRound","iota_types::committee::EpochId","iota_types::committee::StakeUnit","iota_types::messages_checkpoint::CheckpointSequenceNumber","iota_types::messages_checkpoint::CheckpointTimestamp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MoveTypeTagTrait-for-u64\" class=\"impl\"><a class=\"src rightside\" href=\"src/iota_types/lib.rs.html#231-235\">source</a><a href=\"#impl-MoveTypeTagTrait-for-u64\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"iota_types/trait.MoveTypeTagTrait.html\" title=\"trait iota_types::MoveTypeTagTrait\">MoveTypeTagTrait</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u64.html\">u64</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_type_tag\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/iota_types/lib.rs.html#232-234\">source</a><a href=\"#method.get_type_tag\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"iota_types/trait.MoveTypeTagTrait.html#tymethod.get_type_tag\" class=\"fn\">get_type_tag</a>() -&gt; <a class=\"enum\" href=\"iota_types/enum.TypeTag.html\" title=\"enum iota_types::TypeTag\">TypeTag</a></h4></section></div></details>","MoveTypeTagTrait","iota_types::base_types::TxSequenceNumber","iota_types::base_types::CommitRound","iota_types::committee::EpochId","iota_types::committee::StakeUnit","iota_types::messages_checkpoint::CheckpointSequenceNumber","iota_types::messages_checkpoint::CheckpointTimestamp"]],
"iota_util_mem":[]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()