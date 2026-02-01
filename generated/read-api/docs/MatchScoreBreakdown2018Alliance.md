# MatchScoreBreakdown2018Alliance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adjust_points** | Option<**i32**> |  | [optional]
**auto_ownership_points** | **i32** |  | 
**auto_points** | **i32** |  | 
**auto_quest_ranking_point** | Option<**bool**> |  | [optional]
**auto_robot1** | Option<[**models::AutoRobot2018**](AutoRobot2018.md)> |  | [optional]
**auto_robot2** | Option<[**models::AutoRobot2018**](AutoRobot2018.md)> |  | [optional]
**auto_robot3** | Option<[**models::AutoRobot2018**](AutoRobot2018.md)> |  | [optional]
**auto_run_points** | **i32** |  | 
**auto_scale_ownership_sec** | **i32** |  | 
**auto_switch_at_zero** | Option<**bool**> |  | [optional]
**auto_switch_ownership_sec** | **i32** |  | 
**endgame_points** | **i32** |  | 
**endgame_robot1** | Option<[**models::EndgameRobot2018**](EndgameRobot2018.md)> |  | [optional]
**endgame_robot2** | Option<[**models::EndgameRobot2018**](EndgameRobot2018.md)> |  | [optional]
**endgame_robot3** | Option<[**models::EndgameRobot2018**](EndgameRobot2018.md)> |  | [optional]
**face_the_boss_ranking_point** | **bool** |  | 
**foul_count** | Option<**i32**> |  | [optional]
**foul_points** | **i32** |  | 
**rp** | **i32** |  | 
**tech_foul_count** | Option<**i32**> |  | [optional]
**teleop_ownership_points** | **i32** |  | 
**teleop_points** | **i32** |  | 
**teleop_scale_boost_sec** | **i32** |  | 
**teleop_scale_force_sec** | Option<**i32**> |  | [optional]
**teleop_scale_ownership_sec** | **i32** |  | 
**teleop_switch_boost_sec** | **i32** |  | 
**teleop_switch_force_sec** | Option<**i32**> |  | [optional]
**teleop_switch_ownership_sec** | **i32** |  | 
**total_points** | **i32** |  | 
**vault_boost_played** | **i32** |  | 
**vault_boost_total** | **i32** |  | 
**vault_force_played** | **i32** |  | 
**vault_force_total** | **i32** |  | 
**vault_levitate_played** | **i32** |  | 
**vault_levitate_total** | **i32** |  | 
**vault_points** | **i32** |  | 
**tba_game_data** | Option<**TbaGameData**> | Unofficial TBA-computed value of the FMS provided GameData given to the alliance teams at the start of the match. 3 Character String containing `L` and `R` only. The first character represents the near switch, the 2nd the scale, and the 3rd the far, opposing, switch from the alliance's perspective. An `L` in a position indicates the platform on the left will be lit for the alliance while an `R` will indicate the right platform will be lit for the alliance. See also [WPI Screen Steps](https://wpilib.screenstepslive.com/s/currentCS/m/getting_started/l/826278-2018-game-data-details). (enum: , LLL, LRL, RLR, RRR) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


