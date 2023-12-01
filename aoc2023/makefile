startday:
	echo ${day}
	cargo new day-${day}
	rm -rf ./day-${day}/src
	cp -r _template/input ./day-${day}/
	cp -r _template/src ./day-${day}/
	cp -r _template/benches ./day-${day}/
	cd day-${day}; \
	find . -type f -exec sed -i 's/DAYVAR/day_${day}/g' {} +

bench:
	cd day-${day}; \
	cargo bench --bench example >> bench.txt