startday:
	echo ${day}
	cargo new day_${day}
	rm -rf ./day_${day}/src
	rm ./day_${day}/Cargo.toml
	cp -r _template/input ./day_${day}/
	cp -r _template/src ./day_${day}/
	cp -r _template/benches ./day_${day}/
	cp -r _template/Cargo.toml ./day_${day}/
	cd day_${day}; \
	find . -type f -exec sed -i 's/DAYVAR/day_${day}/g' {} +

bench:
	cd day_${day}; \
	cargo bench --bench example >> bench.txt