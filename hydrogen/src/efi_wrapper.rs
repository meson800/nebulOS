use r_efi::efi;

pub fn to_ucs2(in_str: &str, buf: &mut [u16]) -> () {
    let bytes = in_str.as_bytes();
    let len = bytes.len();
    let buf_len = buf.len();
    if len + 1 > buf_len {
	panic!();
    }
    
    let mut i = 0;
    let mut j = 0;

    while i < len {
	let ch;
        if bytes[i] & 0b1000_0000 == 0b0000_0000 {
            ch = u16::from(bytes[i]);
            i += 1;
        } else if bytes[i] & 0b1110_0000 == 0b1100_0000 {
            // 2 byte codepoint
            if i + 1 == len {
                // Buffer underflow
		panic!();
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                // Invalid data
		panic!();
            }
            let a = u16::from(bytes[i] & 0b0001_1111);
            let b = u16::from(bytes[i + 1] & 0b0011_1111);
            ch = a << 6 | b;
            i += 2;
        } else if bytes[i] & 0b1111_0000 == 0b1110_0000 {
            // 3 byte codepoint
            if i + 2 >= len {
		panic!();
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000
                || bytes[i + 2] & 0b1100_0000 != 0b1000_0000
            {
                // Invalid data
		panic!();
            }
            let a = u16::from(bytes[i] & 0b0000_1111);
            let b = u16::from(bytes[i + 1] & 0b0011_1111);
            let c = u16::from(bytes[i + 2] & 0b0011_1111);
            ch = a << 12 | b << 6 | c;
            i += 3;
        } else if bytes[i] & 0b1111_0000 == 0b1111_0000 {
	    panic!();
        } else {
	    panic!();
        }
	buf[j] = ch;
	j += 1;
    }
    buf[j] = 0x0000u16;
}


pub struct BootState {
    systable: *mut efi::SystemTable,
    loaded_device_root: efi::Handle, // stores the device root for the device our image was loaded from
    loaded_image_protocol: *mut efi::protocols::loaded_image::Protocol
}

impl BootState {
    pub fn clear_screen(&mut self) -> efi::Status {
        unsafe {
            ((*(*self.systable).con_out).clear_screen)((*self.systable).con_out)
        }
    }

    pub fn write_string(&mut self, in_str: &str) -> efi::Status {
        let mut buf = [0x0000u16; 1024];
        to_ucs2(in_str, &mut buf);

        // Set the last buffer entry equal to a null terminator
        //buf[-1] = 0x0000u16;

        // and call the display function
        unsafe {
            ((*(*self.systable).con_out).output_string)
            ((*self.systable).con_out, buf.as_ptr() as *mut efi::Char16)
        }
    }

    pub fn from(im: &efi::Handle, st: *mut efi::SystemTable) -> BootState {

        let im_protocol = core::ptr::null_mut() as *mut efi::protocols::loaded_image::Protocol;
        let dev_handle: efi::Handle;
        let r;
        unsafe {
            let null_handle = core::ptr::null_mut() as *mut core::ffi::c_void;
            let mut void_im_ptr = im_protocol as *mut core::ffi::c_void;
            r = ((*(*st).boot_services).open_protocol)(
                *im,
                &mut efi::protocols::loaded_image::PROTOCOL_GUID,
                &mut void_im_ptr,
                *im,
                null_handle,
                0x00000020u32); //exclusive protocol open
            dev_handle = (*im_protocol).device_handle;
        }

        if r.is_error() {
            panic!()
        }

        BootState{systable: st,
                loaded_device_root: dev_handle,
                loaded_image_protocol: im_protocol
                
                }
    }
}

pub fn clear_screen(st: *mut efi::SystemTable) -> efi::Status {
    unsafe {
        ((*(*st).con_out).clear_screen)((*st).con_out)
    }
}
