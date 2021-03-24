export CEPH_ARGS="--bluestore-block-db-size=84825604096"
t=3
for i in 2 3 4 5 6 7 8 9 10 13; do ceph-bluestore-tool bluefs-bdev-new-db --path /var/lib/ceph/osd/ceph-$i/ --dev-target /dev/cache/db-$t;let t=t+1; ceph-bluestore-tool set-label-key --dev /var/lib/ceph/osd/ceph-$i/block -k bluefs-db -v /var/lib/ceph/osd/ceph-$i/block.db;done