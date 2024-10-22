### Creating SampleBuffers
- [x] CMSampleBufferCreateReady
- [ ] CMSampleBufferCreateReadyWithImageBuffer
- [ ] CMAudioSampleBufferCreateReadyWithPacketDescriptions
- [ ] CMSampleBufferCreateWithMakeDataReadyHandler
- [ ] CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler
- [ ] CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler
- [x] CMSampleBufferCreate
- [ ] CMSampleBufferCreateForImageBuffer
- [ ] CMAudioSampleBufferCreateWithPacketDescriptions

### Copying SampleBuffers
- [ ] CMSampleBufferCreateCopy
- [ ] CMSampleBufferCreateCopyWithNewTiming
- [ ] CMSampleBufferCopySampleBufferForRange

### Determeing readyness
- [ ] CMSampleBufferDataIsReady
- [ ] CMSampleBufferSetDataReady
- [ ] CMSampleBufferSetDataFailed
- [ ] CMSampleBufferHasDataFailed
- [x] CMSampleBufferMakeDataReady
- [ ] CMSampleBufferTrackDataReadiness

### Invalidating Sample SampleBuffers
- [ ] CMSampleBufferSetInvalidateHandler
- [ ] CMSampleBufferInvalidate
- [ ] CMSampleBufferIsValid
- [ ] CMSampleBufferSetInvalidateCallback

### Inspecting size information
- [x] CMSampleBufferGetNumSamples
- [x] CMSampleBufferGetTotalSampleSize
- [x] CMSampleBufferGetSampleSize
- [x] CMSampleBufferGetSampleSizeArray

### Inspecting Duration And Timing
- [ ] CMSampleBufferGetDuration
- [ ] CMSampleBufferGetDecodeTimeStamp
- [ ] CMSampleBufferGetPresentationTime
- [ ] CMSampleBufferGetOutputDuration
- [ ] CMSampleBufferGetOutputDecodeTime
- [ ] CMSampleBufferGetOutputPresentationTimeStamp
- [ ] CMSampleBufferSetOutputPresentationTimeStamp
- [ ] CMSampleBufferGetSampleTimingInfo
- [ ] CMSampleBufferGetSampleTimingInfoArray
- [ ] CMSampleBufferGetOutputSampleTimingInfoArray

### Accessing the Format Description
- [x] CMSampleBufferGetFormatDescription

### Modifying Sample Buffer
- [ ] CMSampleBufferGetDataBuffer
- [ ] CMSampleBufferSetDataBuffer
- [ ] CMSampleBufferGetImageBuffer
- [ ] CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer
- [ ] CMSampleBufferSetDataBufferFromAudioBufferList
- [ ] CMSampleBufferCopyPCMDataIntoAudioBufferList
- [ ] CMSampleBufferGetAudioStreamPacketDescriptions
- [ ] CMSampleBufferGetAudioStreamPacketDescriptionsPtr

### Managing Attachments
- [ ] CMSampleBufferGetSampleAttachmentsArray

### Processing Samples
- [ ] CMSampleBufferCallBlockForEachSample
- [ ] CMSampleBufferCallForEachSample

