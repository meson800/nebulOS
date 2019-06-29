#!/bin/bash
rm -r temp_rundir
mkdir -p temp_rundir
mkdir -p temp_rundir/EFI/Boot/
cp hydrogen/target/x86_64-unknown-uefi/debug/nebulos-hydrogen.efi temp_rundir/EFI/Boot/BootX64.efi
qemu-system-x86_64 -L . -drive format=raw,file=/usr/share/ovmf/OVMF.fd,readonly=on,if=pflash -drive format=raw,file=fat:temp_rundir,index=0,media=disk -net none
