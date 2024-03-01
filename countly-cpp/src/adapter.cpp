#include "adapter.hpp"
#include "countly.hpp"
#include "countly/logger_module.hpp"
#include <iostream>

using namespace cly;
using namespace std;


void printLog(LogLevel level, const string& msg) {
    std::cout << msg << "\n";
}

void CLY_SetDeviceId(const char* deviceId) {
    Countly& countly = Countly::getInstance();
    countly.setDeviceID(deviceId);
}


void CLY_SetMetrics(const char* os,const char* os_version,const char* device,const char* resolution,const char* carrier,const char* app_version) {
    Countly& countly = Countly::getInstance();
    countly.setMetrics(os, os_version, device, resolution, carrier, app_version);
}

void CLY_Start(const char* appkey, const char* host, int port) {
    Countly& countly = Countly::getInstance();
    countly.start(appkey, host, port, true);
}

void CLY_RecordEventCount(const char* key, int count, MapEntry* mapEntry, size_t entrySize){
    Countly& countly = Countly::getInstance();
    Event event(key, count);

    if(entrySize > 0){
        for(size_t i = 0; i < entrySize; i++){
            MapEntry* entry = mapEntry + i;
            event.addSegmentation(entry->key,entry->value);
        }   
    }
    countly.addEvent(event);
}
void CLY_RecordEventCountSum(const char* key, int count,double sum,MapEntry* mapEntry, size_t entrySize) {
    Countly& countly = Countly::getInstance();
    Event event(key, count,sum);

    if(entrySize > 0){
        for(size_t i = 0; i < entrySize; i++){
            MapEntry* entry = mapEntry + i;
            event.addSegmentation(entry->key,entry->value);
        }   
    }
    countly.addEvent(event);
}
void CLY_RecordEventCountSumDuration(const char* key,int count,double sum,double duration,MapEntry* mapEntry, size_t entrySize) {
    Countly& countly = Countly::getInstance();
    Event event(key, count, sum, duration);

    if(entrySize > 0){
        for(size_t i = 0; i < entrySize; i++){
            MapEntry* entry = mapEntry + i;
            event.addSegmentation(entry->key,entry->value);
        }   
    }
    countly.addEvent(event);
}


void CLY_FlushEvents(void){
    Countly& countly = Countly::getInstance();
    countly.flushEvents();
}

int CLY_OpenView(const char* name, MapEntry* mapEntry, size_t entrySize, ViewId* viewId) {
    Countly& countly = Countly::getInstance();
    std::map <std::string, std::string> segments;

    if(entrySize > 0){
        for(size_t i = 0; i < entrySize; i++){
            MapEntry* entry = mapEntry + i;
            segments[entry->key] = entry->value;
        }   
    }
    std::string id = countly.views().openView(name,segments);
    size_t length = id.length();

    if(length < sizeof(ViewId) - 1){
        memcpy(viewId, id.c_str(), length);
        *((char *)viewId + length) = '\0';
        return 0;
    }else{
        countly.views().closeViewWithID(id);
        return 1;
    }

}

void CLY_CloseViewWithId(const ViewId viewId) {
    Countly& countly = Countly::getInstance();
    countly.views().closeViewWithID(string((char*)viewId));
}


