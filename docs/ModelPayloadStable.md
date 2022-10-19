# ModelPayloadStable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sampler_name** | Option<**String**> |  | [optional][default to KEuler]
**toggles** | Option<**Vec<i32>**> | Special Toggles used in the SD Webui. To be documented. | [optional]
**cfg_scale** | Option<**f32**> |  | [optional]
**denoising_strength** | Option<**f32**> |  | [optional]
**seed** | Option<**String**> | The seed to use to generete this request | [optional]
**height** | Option<**i32**> | The height of the image to generate | [optional]
**width** | Option<**i32**> | The width of the image to generate | [optional]
**seed_variation** | Option<**i32**> | If passed with multiple n, the provided seed will be incremented every time by this value | [optional]
**use_gfpgan** | Option<**bool**> | Set to true to process the generated image with GFPGAN (face correction) | [optional]
**use_real_esrgan** | Option<**bool**> | Set to true to process the generated image with RealESRGAN | [optional]
**use_ldsr** | Option<**bool**> | Set to true to process the generated image with LDSR | [optional]
**use_upscaling** | Option<**bool**> | Set to true to upscale the image | [optional]
**prompt** | Option<**String**> | The prompt which will be sent to Stable Diffusion to generate an image | [optional]
**ddim_steps** | Option<**i32**> |  | [optional]
**n_iter** | Option<**i32**> | The amount of images to generate | [optional]
**use_nsfw_censor** | Option<**bool**> | When true will apply NSFW censoring model on the generation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


