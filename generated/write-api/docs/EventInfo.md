# EventInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_code** | Option<**String**> | Event code used to sync data with FIRST | [optional]
**playoff_type** | Option<**i32**> | Integer constant representing the playoff format. References constants defined under `PlayoffType`: https://github.com/the-blue-alliance/the-blue-alliance/blob/main/src/backend/common/consts/playoff_type.py#L37 | [optional]
**webcasts** | Option<[**Vec<models::EventInfoWebcastsInner>**](EventInfo_webcasts_inner.md)> | A list of webcast URLs to set for this event. This will overwrite the existing webcast list | [optional]
**remap_teams** | Option<**std::collections::HashMap<String, String>**> | A mapping of temp key --> remapped key (including B team keys) | [optional]
**timezone** | Option<**String**> | Timezone name for the event | [optional]
**disable_sync** | Option<[**models::EventInfoDisableSync**](EventInfo_disable_sync.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


