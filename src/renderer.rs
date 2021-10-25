use ash::{Entry, vk};
use winit::window::Window;
use std::ffi::{CString, CStr};

pub struct Renderer {
    entry: Entry,
    instance: ash::Instance,
}

impl Renderer {
    pub fn new(window : &Window) -> Renderer {
        let entry = unsafe { Entry::new().expect("Entry creation error") };
        Renderer::enumerate_available_extensions(&entry);
        let instance = Renderer::create_instance(window, &entry);
        Renderer { 
            entry,
            instance,
        }
    }

    fn get_required_layers() -> Vec<* const i8> {
        let layers = vec![CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").expect("Wrong layer string")];
        layers.iter().map(|l| l.as_ptr()).collect()
    }

    fn get_required_extensions(window : &Window) -> Vec<*const i8> {
        let extensions = ash_window::enumerate_required_extensions(window).unwrap();
        extensions.iter().map(|extension| extension.as_ptr()).collect()
    }

    fn create_instance(window : &Window, entry: &Entry) -> ash::Instance {
        let app_name = CString::new("kdrvr").unwrap();
        let application_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let layers = Renderer::get_required_layers();
        let extensions = Renderer::get_required_extensions(window);
        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions);
        unsafe {entry.create_instance(&instance_create_info, None).expect("Instance creation error")}
    }

    fn enumerate_available_extensions(entry: &Entry) {
        if let Ok(extensions) = entry.enumerate_instance_extension_properties() {
            for extension in extensions {
                println!("{:?}", extension);
            } 
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
