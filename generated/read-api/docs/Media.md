# Media

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | String type of the media element. (enum: youtube, cdphotothread, imgur, facebook-profile, youtube-channel, twitter-profile, github-profile, instagram-profile, periscope-profile, gitlab-profile, grabcad, instagram-image, external-link, avatar, onshape, cd-thread) | 
**foreign_key** | **String** | The key used to identify this media on the media site. | 
**details** | Option<[**models::MediaDetails**](MediaDetails.md)> |  | [optional]
**preferred** | Option<**bool**> | True if the media is of high quality. | [optional]
**team_keys** | **Vec<String>** | List of teams that this media belongs to. Most likely length 1. | 
**direct_url** | Option<**String**> | Direct URL to the media. | [optional]
**view_url** | Option<**String**> | The URL that leads to the full web page for the media, if one exists. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


