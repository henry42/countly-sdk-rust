use std::{collections::HashMap, ffi::{CStr, CString}};
use url::Url;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/rustc_ver.rs"));


pub fn map_segmentation<T:FnOnce(*mut MapEntry,usize)->R,R>(segmentation:Option<HashMap<String,String>>, callback:T) -> R{
    unsafe {
        let mut map_entries: Vec<MapEntry> = Vec::new();

        if let Some(segmentation) = segmentation {
            for (key, value) in segmentation {
                let key_c = CString::new(key).unwrap();
                let value_c = CString::new(value).unwrap();
                map_entries.push(MapEntry {
                    key: key_c.into_raw(),
                    value: value_c.into_raw()
                });
            }
        }

        let mut boxed_entries = map_entries.into_boxed_slice();
        let entries_ptr = boxed_entries.as_mut_ptr();

        let r = callback(entries_ptr,boxed_entries.len());

        for seg_entry in boxed_entries.into_iter() {
            let _ = CString::from_raw(seg_entry.key);
            let _ = CString::from_raw(seg_entry.value);
        }

        r
    }
}


pub fn cly_set_device_id(device_id: &str){
    unsafe {
        let device_id_c = CString::new(device_id).unwrap();
        CLY_SetDeviceId(device_id_c.as_ptr().cast());
    }
}

pub fn cly_set_default_metrics(){
    cly_set_metrics(std::env::consts::OS, RUSTC_VERSION, std::env::consts::ARCH, "1080x1920", RUSTC_HOST, PKG_VERSION);
}

pub fn cly_set_metrics(os: &str, os_version: &str, device: &str, resolution: &str, carrier: &str, app_version: &str){
    unsafe {
        let os_c = CString::new(os).unwrap();
        let os_version_c = CString::new(os_version).unwrap();
        let device_c = CString::new(device).unwrap();
        let resolution_c = CString::new(resolution).unwrap();
        let carrier_c = CString::new(carrier).unwrap();
        let app_version_c = CString::new(app_version).unwrap();
        CLY_SetMetrics(os_c.as_ptr().cast(), os_version_c.as_ptr().cast(), device_c.as_ptr().cast(), resolution_c.as_ptr().cast(), carrier_c.as_ptr().cast(), app_version_c.as_ptr().cast());
    }
}

pub fn cly_start(app_key: &str, url: &str) {

    let mut url = Url::parse(url).unwrap();
    let port = url.port().unwrap_or_else(|| if url.scheme() == "https" { 443 } else { 80 } );
    let _ = url.set_port(None);
    let host = format!("{}://{}", url.scheme(),url.host_str().unwrap());

    unsafe {
        let app_key_c = CString::new(app_key).unwrap();
        let host_c = CString::new(host).unwrap();
        CLY_Start(app_key_c.as_ptr().cast(), host_c.as_ptr().cast(), port.try_into().unwrap_or(443));
    }
}

pub fn cly_record_event_count(event: &str, count: u32, segmentation:Option<HashMap<String,String>>) {
    cly_record_event_count_sum_duration(event, count, 0.0, 0.0, segmentation);
}

pub fn cly_record_event_count_sum(event: &str, count: u32, sum: f64,segmentation:Option<HashMap<String,String>>) {
    cly_record_event_count_sum_duration(event, count, sum, 0.0, segmentation);
}

pub fn cly_record_event_count_sum_duration(event: &str, count: u32, sum: f64, duration: f64,segmentation:Option<HashMap<String,String>>) {
    unsafe {
        let event_c = CString::new(event).unwrap();
        map_segmentation(segmentation, |entries_ptr,size| {
            if sum == 0.0 && duration == 0.0 {
                CLY_RecordEventCount(
                    event_c.as_ptr().cast(), 
                    count.try_into().unwrap_or(0), 
                    entries_ptr, 
                    size);
            }else if duration == 0.0 {
                CLY_RecordEventCountSum(
                    event_c.as_ptr().cast(),
                    count.try_into().unwrap_or(0),
                    sum,
                    entries_ptr,
                    size
                );
            }else{
                CLY_RecordEventCountSumDuration(
                    event_c.as_ptr().cast(), 
                    count.try_into().unwrap_or(0),
                    sum,
                    duration,
                    entries_ptr,
                    size
                );
            }
        }); 
    }
}

pub fn cly_flush_events(){
    unsafe{
        CLY_FlushEvents();
    }
}

pub fn cly_open_view(name:&str,segmentation:Option<HashMap<String,String>>) -> Option<String>{
    let name_c = CString::new(name).unwrap();
    unsafe{
        map_segmentation(segmentation, |entries_ptr,size| {
            let mut view_id:ViewId = std::mem::zeroed();
            let view_id_ptr = &mut view_id as *mut ViewId;
            if CLY_OpenView(name_c.as_ptr().cast(), entries_ptr, size, view_id_ptr) == 0 {
                Some(CStr::from_ptr(view_id_ptr.cast()).to_string_lossy().to_string())
            }else{
                None
            }
        })
    }
}

pub fn cly_close_view(view_id: &str){
    unsafe{
        CLY_CloseViewWithId(view_id.as_ptr() as *mut i8);
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn countly_events(){
        let app_key = std::env::var("COUNTLY_APP_KEY").expect("Need COUNTLY_APP_KEY");
        let url = std::env::var("COUNTLY_URL").expect("Need COUNTLY_URL");

        let segmentation= HashMap::from([
            ("Mercury", "a"),
            ("Venus", "b"),
            ("Earth", "c"),
            ("Mars", "d"),
        ]).into_iter().map(|f|(f.0.to_string(),f.1.to_string())).collect();

        cly_set_device_id("aaa");
        cly_set_default_metrics();
        cly_start(app_key.as_str(), url.as_str());
        cly_record_event_count("rust event", 10, Some(segmentation));
        cly_flush_events();
    }

    #[test]
    fn countly_openview(){
        let app_key = std::env::var("COUNTLY_APP_KEY").expect("Need COUNTLY_APP_KEY");
        let url = std::env::var("COUNTLY_URL").expect("Need COUNTLY_URL");

        let segmentation= HashMap::from([
            ("Mercury", "a"),
            ("Venus", "b"),
            ("Earth", "c"),
            ("Mars", "d"),
        ]).into_iter().map(|f|(f.0.to_string(),f.1.to_string())).collect();

        cly_set_device_id("aaa");
        cly_set_default_metrics();
        cly_start(app_key.as_str(), url.as_str());
        let x = cly_open_view("rust view", Some(segmentation));
        assert!(x.is_some());
        sleep(Duration::from_secs(5));
        cly_close_view(x.unwrap().as_str());
        cly_flush_events();
    }
}
