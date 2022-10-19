# WorkerDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The Name given to this worker. | [optional]
**id** | Option<**String**> | The UUID of this worker. | [optional]
**requests_fulfilled** | Option<**i32**> | How many images this worker has generated. | [optional]
**kudos_rewards** | Option<**f32**> | How many Kudos this worker has been rewarded in total. | [optional]
**kudos_details** | Option<[**crate::models::WorkerKudosDetails**](WorkerKudosDetails.md)> |  | [optional]
**performance** | Option<**String**> | The average performance of this worker in human readable form. | [optional]
**uptime** | Option<**i32**> | The amount of seconds this worker has been online for this Horde. | [optional]
**maintenance_mode** | Option<**bool**> | When True, this worker will not pick up any new requests | [optional]
**paused** | Option<**bool**> | (Privileged) When True, this worker not be given any new requests. | [optional]
**info** | Option<**String**> | Extra information or comments about this worker provided by its owner. | [optional]
**nsfw** | Option<**bool**> | Whether this worker can generate NSFW requests or not. | [optional][default to false]
**owner** | Option<**String**> | Privileged or public if the owner has allowed it. The alias of the owner of this worker. | [optional]
**trusted** | Option<**bool**> | The worker is trusted to return valid generations. | [optional]
**suspicious** | Option<**i32**> | (Privileged) How much suspicion this worker has accumulated | [optional]
**models** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


