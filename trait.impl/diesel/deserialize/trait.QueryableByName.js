(function() {var implementors = {
"iota_indexer":[["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/address_metrics/struct.StoredAddressMetrics.html\" title=\"struct iota_indexer::models::address_metrics::StoredAddressMetrics\">StoredAddressMetrics</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/events/struct.StoredEvent.html\" title=\"struct iota_indexer::models::events::StoredEvent\">StoredEvent</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;&gt;&gt;: FromSql&lt;Array&lt;Nullable&lt;Bytea&gt;&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/move_call_metrics/struct.QueriedMoveMetrics.html\" title=\"struct iota_indexer::models::move_call_metrics::QueriedMoveMetrics\">QueriedMoveMetrics</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/network_metrics/struct.StoredNetworkMetrics.html\" title=\"struct iota_indexer::models::network_metrics::StoredNetworkMetrics\">StoredNetworkMetrics</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.f64.html\">f64</a>: FromSql&lt;Double, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/network_metrics/struct.Tps.html\" title=\"struct iota_indexer::models::network_metrics::Tps\">Tps</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.f64.html\">f64</a>: FromSql&lt;Float8, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/obj_indices/struct.StoredObjectVersion.html\" title=\"struct iota_indexer::models::obj_indices::StoredObjectVersion\">StoredObjectVersion</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_version/dsl/struct.object_id.html\" title=\"struct iota_indexer::schema::objects_version::dsl::object_id\">object_id</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_version/dsl/struct.object_version.html\" title=\"struct iota_indexer::schema::objects_version::dsl::object_version\">object_version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_version/dsl/struct.cp_sequence_number.html\" title=\"struct iota_indexer::schema::objects_version::dsl::cp_sequence_number\">cp_sequence_number</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/objects/struct.CoinBalance.html\" title=\"struct iota_indexer::models::objects::CoinBalance\">CoinBalance</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/objects/struct.StoredDeletedHistoryObject.html\" title=\"struct iota_indexer::models::objects::StoredDeletedHistoryObject\">StoredDeletedHistoryObject</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_id.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_id\">object_id</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_version.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_version\">object_version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.checkpoint_sequence_number.html\" title=\"struct iota_indexer::schema::objects_history::dsl::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_status.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_status\">object_status</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/objects/struct.StoredDeletedObject.html\" title=\"struct iota_indexer::models::objects::StoredDeletedObject\">StoredDeletedObject</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_id.html\" title=\"struct iota_indexer::schema::objects::dsl::object_id\">object_id</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_version.html\" title=\"struct iota_indexer::schema::objects::dsl::object_version\">object_version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.checkpoint_sequence_number.html\" title=\"struct iota_indexer::schema::objects::dsl::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/objects/struct.StoredHistoryObject.html\" title=\"struct iota_indexer::models::objects::StoredHistoryObject\">StoredHistoryObject</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_id.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_id\">object_id</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_version.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_version\">object_version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.checkpoint_sequence_number.html\" title=\"struct iota_indexer::schema::objects_history::dsl::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_status.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_status\">object_status</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_digest.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_digest\">object_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.owner_id.html\" title=\"struct iota_indexer::schema::objects_history::dsl::owner_id\">owner_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_type_package.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_type_package\">object_type_package</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.serialized_object.html\" title=\"struct iota_indexer::schema::objects_history::dsl::serialized_object\">serialized_object</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.df_name.html\" title=\"struct iota_indexer::schema::objects_history::dsl::df_name\">df_name</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.df_object_id.html\" title=\"struct iota_indexer::schema::objects_history::dsl::df_object_id\">df_object_id</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.owner_type.html\" title=\"struct iota_indexer::schema::objects_history::dsl::owner_type\">owner_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.df_kind.html\" title=\"struct iota_indexer::schema::objects_history::dsl::df_kind\">df_kind</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_type.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_type\">object_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_type_module.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_type_module\">object_type_module</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.object_type_name.html\" title=\"struct iota_indexer::schema::objects_history::dsl::object_type_name\">object_type_name</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.coin_type.html\" title=\"struct iota_indexer::schema::objects_history::dsl::coin_type\">coin_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.df_object_type.html\" title=\"struct iota_indexer::schema::objects_history::dsl::df_object_type\">df_object_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_history/dsl/struct.coin_balance.html\" title=\"struct iota_indexer::schema::objects_history::dsl::coin_balance\">coin_balance</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/objects/struct.StoredObject.html\" title=\"struct iota_indexer::models::objects::StoredObject\">StoredObject</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_id.html\" title=\"struct iota_indexer::schema::objects::dsl::object_id\">object_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_digest.html\" title=\"struct iota_indexer::schema::objects::dsl::object_digest\">object_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.serialized_object.html\" title=\"struct iota_indexer::schema::objects::dsl::serialized_object\">serialized_object</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_version.html\" title=\"struct iota_indexer::schema::objects::dsl::object_version\">object_version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.checkpoint_sequence_number.html\" title=\"struct iota_indexer::schema::objects::dsl::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.owner_type.html\" title=\"struct iota_indexer::schema::objects::dsl::owner_type\">owner_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.owner_id.html\" title=\"struct iota_indexer::schema::objects::dsl::owner_id\">owner_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_type_package.html\" title=\"struct iota_indexer::schema::objects::dsl::object_type_package\">object_type_package</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.df_name.html\" title=\"struct iota_indexer::schema::objects::dsl::df_name\">df_name</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.df_object_id.html\" title=\"struct iota_indexer::schema::objects::dsl::df_object_id\">df_object_id</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_type.html\" title=\"struct iota_indexer::schema::objects::dsl::object_type\">object_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_type_module.html\" title=\"struct iota_indexer::schema::objects::dsl::object_type_module\">object_type_module</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.object_type_name.html\" title=\"struct iota_indexer::schema::objects::dsl::object_type_name\">object_type_name</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.coin_type.html\" title=\"struct iota_indexer::schema::objects::dsl::coin_type\">coin_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.df_object_type.html\" title=\"struct iota_indexer::schema::objects::dsl::df_object_type\">df_object_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.coin_balance.html\" title=\"struct iota_indexer::schema::objects::dsl::coin_balance\">coin_balance</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects/dsl/struct.df_kind.html\" title=\"struct iota_indexer::schema::objects::dsl::df_kind\">df_kind</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/objects/struct.StoredObjectSnapshot.html\" title=\"struct iota_indexer::models::objects::StoredObjectSnapshot\">StoredObjectSnapshot</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_id.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_id\">object_id</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_version.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_version\">object_version</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.checkpoint_sequence_number.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_status.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_status\">object_status</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_digest.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_digest\">object_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.owner_id.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::owner_id\">owner_id</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_type_package.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_type_package\">object_type_package</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.serialized_object.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::serialized_object\">serialized_object</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.df_name.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::df_name\">df_name</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.df_object_id.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::df_object_id\">df_object_id</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.owner_type.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::owner_type\">owner_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.df_kind.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::df_kind\">df_kind</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_type.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_type\">object_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_type_module.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_type_module\">object_type_module</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.object_type_name.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::object_type_name\">object_type_name</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.coin_type.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::coin_type\">coin_type</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.df_object_type.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::df_object_type\">df_object_type</a>&gt;, __DB&gt;,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/objects_snapshot/dsl/struct.coin_balance.html\" title=\"struct iota_indexer::schema::objects_snapshot::dsl::coin_balance\">coin_balance</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/transactions/struct.StoredTransaction.html\" title=\"struct iota_indexer::models::transactions::StoredTransaction\">StoredTransaction</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.tx_sequence_number.html\" title=\"struct iota_indexer::schema::transactions::dsl::tx_sequence_number\">tx_sequence_number</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.checkpoint_sequence_number.html\" title=\"struct iota_indexer::schema::transactions::dsl::checkpoint_sequence_number\">checkpoint_sequence_number</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.timestamp_ms.html\" title=\"struct iota_indexer::schema::transactions::dsl::timestamp_ms\">timestamp_ms</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.transaction_digest.html\" title=\"struct iota_indexer::schema::transactions::dsl::transaction_digest\">transaction_digest</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.raw_transaction.html\" title=\"struct iota_indexer::schema::transactions::dsl::raw_transaction\">raw_transaction</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.raw_effects.html\" title=\"struct iota_indexer::schema::transactions::dsl::raw_effects\">raw_effects</a>&gt;, __DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;&gt;&gt;: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.object_changes.html\" title=\"struct iota_indexer::schema::transactions::dsl::object_changes\">object_changes</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.balance_changes.html\" title=\"struct iota_indexer::schema::transactions::dsl::balance_changes\">balance_changes</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.events.html\" title=\"struct iota_indexer::schema::transactions::dsl::events\">events</a>&gt;, __DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i16.html\">i16</a>: FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.transaction_kind.html\" title=\"struct iota_indexer::schema::transactions::dsl::transaction_kind\">transaction_kind</a>&gt;, __DB&gt; + FromSql&lt;SqlTypeOf&lt;<a class=\"struct\" href=\"iota_indexer/schema/transactions/dsl/struct.success_command_count.html\" title=\"struct iota_indexer::schema::transactions::dsl::success_command_count\">success_command_count</a>&gt;, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/tx_indices/struct.TxDigest.html\" title=\"struct iota_indexer::models::tx_indices::TxDigest\">TxDigest</a><div class=\"where\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, __DB&gt;,</div>"],["impl&lt;__DB: Backend&gt; QueryableByName&lt;__DB&gt; for <a class=\"struct\" href=\"iota_indexer/models/tx_indices/struct.TxSequenceNumber.html\" title=\"struct iota_indexer::models::tx_indices::TxSequenceNumber\">TxSequenceNumber</a><div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, __DB&gt;,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()