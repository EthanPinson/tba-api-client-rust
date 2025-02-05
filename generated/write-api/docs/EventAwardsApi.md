# \EventAwardsApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_awards**](EventAwardsApi.md#update_awards) | **POST** /event/{eventKey}/awards/update | Update the award listing at an event



## update_awards

> update_awards(event_key, award)
Update the award listing at an event

Overwrite the event's award listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**award** | [**Vec<models::Award>**](Award.md) | List of awards. An award is a dict of:  *name_str*: String of award name. ex: \"Tournament Winner\" or \"Dean's List Finalist\"  *team_key*: String in the format `frcXXX` for the team that won the award. Can be null.  *awardee*: String corresponding to the name of an individual that won the award. Can be null. | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

