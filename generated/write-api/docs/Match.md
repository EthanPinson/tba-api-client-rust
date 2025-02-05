# Match

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comp_level** | **String** | Competition level of the match | 
**set_number** | **i32** | Set number of the match | 
**match_number** | **i32** | Number of the match in the current set | 
**alliances** | [**models::MatchAlliances**](Match_alliances.md) |  | 
**score_breakdown** | Option<**String**> | Dict of {'red': {K1: V1, K2: V2, ...}, 'blue': {...}}. Where Kn are keys and Vn are values for those keys. Valid keys Kn vary by year. For 2014, valid keys are: 'auto', 'assist', 'truss+catch', 'teleop_goal+foul'. | [optional]
**time_str** | Option<**String**> | String in the format \"(H)H:MM AM/PM\" for when the match will be played in the event's local timezone | [optional]
**time_utc** | Option<**String**> | UTC time of match in ISO 8601 format (YYYY-MM-DDTHH:MM:SS) | [optional]
**display_name** | Option<**String**> | The name for the match, to be shown on the event page | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


