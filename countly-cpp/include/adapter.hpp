#include <stddef.h>

extern "C" {

    typedef struct {
        char* key;
        char* value;
    } MapEntry;

    typedef char ViewId[50];

    void CLY_SetDeviceId(const char* deviceId);
    void CLY_SetMetrics(const char* os,const char* os_version,const char* device,const char* resolution,const char* carrier,const char* app_version);
    void CLY_Start(const char* appkey, const char* host, int port);
    void CLY_RecordEventCount(const char*, int count, MapEntry* mapEntry, size_t entrySize);
    void CLY_RecordEventCountSum(const char*, int count,double sum,MapEntry* mapEntry, size_t entrySize);
    void CLY_RecordEventCountSumDuration(const char*,int count,double sum,double duration,MapEntry* mapEntry, size_t entrySize);
    void CLY_FlushEvents(void);

    int CLY_OpenView(const char* name, MapEntry* mapEntry, size_t entrySize, ViewId* viewId);
    void CLY_CloseViewWithId(const ViewId viewId);
    
}
