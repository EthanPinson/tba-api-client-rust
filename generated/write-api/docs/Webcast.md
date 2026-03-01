# Webcast

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | Optional URL that will be parsed to extract webcast type and channel. Either provide url OR type+channel+file | [optional]
**r#type** | Option<**String**> | Type of webcast (e.g., youtube, twitch, livestream, http_stream). Required if url is not provided | [optional]
**channel** | Option<**String**> | Channel identifier for the webcast. Required if url is not provided | [optional]
**file** | Option<**String**> | Optional file identifier for certain webcast types | [optional]
**date** | Option<**String**> | Optional date in YYYY-MM-DD format for when the webcast aired | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


