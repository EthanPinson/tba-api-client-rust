# EliminationAllianceStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**playoff_average** | Option<**f64**> | Average match score during playoffs. Year specific. May be null. | [optional]
**playoff_type** | Option<**f64**> | Playoff type, may be null. | 
**level** | [**models::CompLevel**](Comp_Level.md) | Match level, qm/ef/qf/sf/f. | 
**record** | [**models::WltRecord**](WLT_Record.md) | W-L-T record for the alliance, may be null. | 
**current_level_record** | [**models::WltRecord**](WLT_Record.md) | W-L-T record for the alliance at the current level, may be null. | 
**status** | **String** | Status of the alliance. | 
**advanced_to_round_robin_finals** | Option<**bool**> | Whether the alliance advanced to round robin finals. | [optional]
**double_elim_round** | Option<[**models::DoubleElimRound**](Double_Elim_Round.md)> |  | [optional]
**round_robin_rank** | Option<**i32**> | Rank in round robin play. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


