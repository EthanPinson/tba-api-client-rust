# \MatchVideosApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_match_videos**](MatchVideosApi.md#update_match_videos) | **POST** /event/{eventKey}/match_videos/add | Add match videos



## update_match_videos

> update_match_videos(event_key, body)
Add match videos

Link YouTube videos to a match

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**body** | **String** | Dict of {key: partial match key, value: youtube video id} | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

