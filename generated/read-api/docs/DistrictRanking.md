# DistrictRanking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_key** | **String** | TBA team key for the team. | 
**rank** | **i32** | Numerical rank of the team, 1 being top rank. | 
**rookie_bonus** | **i32** | Any points added to a team as a result of the rookie bonus. | 
**point_total** | **i32** | Total district points for the team. | 
**event_points** | [**Vec<models::DistrictRankingEventPointsInner>**](District_Ranking_event_points_inner.md) | List of events that contributed to the point total for the team. | 
**adjustments** | Option<**i32**> | Any points adjustments applied to the team. | [optional]
**other_bonus** | Option<**i32**> | Any other bonus points awarded to the team. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


