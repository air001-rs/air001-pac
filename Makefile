all:
	svd2rust \
		--target cortex-m \
		--pascal_enum_values \
		--max_cluster_size \
		--atomics \
		--atomics_feature atomics \
		-i svd/air001xx.svd
	rm -r src
	form -i lib.rs -o src && rm lib.rs
	cargo fmt
