use crate::model::macros::int_enum;

int_enum! {





















    #[doc(alias = "RESTJSONErrorCodes")]
    pub enum ErrorCode: u32 {

        #[doc(alias = "0")]
        GeneralError = 0,

        #[doc(alias = "10001")]
        UnknownAccount = 10_001,

        #[doc(alias = "10002")]
        UnknownApplication = 10_002,

        #[doc(alias = "10003")]
        UnknownChannel = 10_003,

        #[doc(alias = "10004")]
        UnknownGuild = 10_004,

        #[doc(alias = "10005")]
        UnknownIntegration = 10_005,

        #[doc(alias = "10006")]
        UnknownInvite = 10_006,

        #[doc(alias = "10007")]
        UnknownMember = 10_007,

        #[doc(alias = "10008")]
        UnknownMessage = 10_008,

        #[doc(alias = "10009")]
        UnknownPermissionOverwrite = 10_009,

        #[doc(alias = "10010")]
        UnknownProvider = 10_010,

        #[doc(alias = "10011")]
        UnknownRole = 10_011,

        #[doc(alias = "10012")]
        UnknownToken = 10_012,

        #[doc(alias = "10013")]
        UnknownUser = 10_013,

        #[doc(alias = "10014")]
        UnknownEmoji = 10_014,

        #[doc(alias = "10015")]
        UnknownWebhook = 10_015,

        #[doc(alias = "10016")]
        UnknownWebhookService = 10_016,

        #[doc(alias = "10020")]
        UnknownSession = 10_020,

        #[doc(alias = "10021")]
        UnknownAsset = 10_021,

        #[doc(alias = "10026")]
        UnknownBan = 10_026,

        #[doc(alias = "UnknownSKU")]
        #[doc(alias = "10027")]
        UnknownSku = 10_027,

        #[doc(alias = "10028")]
        UnknownStoreListing = 10_028,

        #[doc(alias = "10029")]
        UnknownEntitlement = 10_029,

        #[doc(alias = "10030")]
        UnknownBuild = 10_030,

        #[doc(alias = "10031")]
        UnknownLobby = 10_031,

        #[doc(alias = "10032")]
        UnknownBranch = 10_032,

        #[doc(alias = "10033")]
        UnknownStoreDirectoryLayout = 10_033,

        #[doc(alias = "10036")]
        UnknownRedistributable = 10_036,

        #[doc(alias = "10038")]
        UnknownGiftCode = 10_038,

        #[doc(alias = "10049")]
        UnknownStream = 10_049,

        #[doc(alias = "10050")]
        UnknownPremiumServerSubscribeCooldown = 10_050,

        #[doc(alias = "10057")]
        UnknownGuildTemplate = 10_057,

        #[doc(alias = "10059")]
        UnknownDiscoverableServerCategory = 10_059,

        #[doc(alias = "10060")]
        UnknownSticker = 10_060,

        #[doc(alias = "10061")]
        UnknownStickerPack = 10_061,

        #[doc(alias = "10062")]
        UnknownInteraction = 10_062,

        #[doc(alias = "10063")]
        UnknownApplicationCommand = 10_063,

        #[doc(alias = "10065")]
        UnknownVoiceState = 10_065,

        #[doc(alias = "10066")]
        UnknownApplicationCommandPermissions = 10_066,

        #[doc(alias = "10067")]
        UnknownStageInstance = 10_067,

        #[doc(alias = "10068")]
        UnknownGuildMemberVerificationForm = 10_068,

        #[doc(alias = "10069")]
        UnknownGuildWelcomeScreen = 10_069,

        #[doc(alias = "10070")]
        UnknownGuildScheduledEvent = 10_070,

        #[doc(alias = "10071")]
        UnknownGuildScheduledEventUser = 10_071,

        #[doc(alias = "10087")]
        UnknownTag = 10_087,

        #[doc(alias = "10097")]
        UnknownSound = 10_097,

        #[doc(alias = "10124")]
        UnknownInviteTargetUsersJob = 10_124,

        #[doc(alias = "10129")]
        UnknownInviteTargetUsers = 10_129,

        #[doc(alias = "20001")]
        BotsCannotUseThisEndpoint = 20_001,

        #[doc(alias = "20002")]
        OnlyBotsCanUseThisEndpoint = 20_002,

        #[doc(alias = "20009")]
        ExplicitContentCannotBeSentToTheDesiredRecipient = 20_009,

        #[doc(alias = "20012")]
        NotAuthorizedToPerformThisActionOnThisApplication = 20_012,

        #[doc(alias = "20015")]
        ThisActionRequiresAPremiumSubscription = 20_015,

        #[doc(alias = "20016")]
        ActionCannotBePerformedDueToSlowmodeRateLimit = 20_016,

        #[doc(alias = "20018")]
        OnlyTheOwnerOfThisAccountCanPerformThisAction = 20_018,

        #[doc(alias = "20022")]
        AnnouncementEditLimitExceeded = 20_022,

        #[doc(alias = "20024")]
        UnderMinimumAge = 20_024,

        #[doc(alias = "20028")]
        ChannelWriteRateLimit = 20_028,

        #[doc(alias = "20029")]
        ServerWriteRateLimit = 20_029,


        #[doc(alias = "20031")]
        StageTopicServerNameServerDescriptionOrChannelNamesContainDisallowedWords = 20_031,

        #[doc(alias = "20035")]
        GuildPremiumSubscriptionLevelTooLow = 20_035,

        #[doc(alias = "30001")]
        MaximumNumberOfGuildsReached = 30_001,

        #[doc(alias = "30002")]
        MaximumNumberOfFriendsReached = 30_002,

        #[doc(alias = "30003")]
        MaximumNumberOfPinsReachedForTheChannel = 30_003,

        #[doc(alias = "30004")]
        MaximumNumberOfRecipientsReached = 30_004,

        #[doc(alias = "30005")]
        MaximumNumberOfGuildRolesReached = 30_005,

        #[doc(alias = "30007")]
        MaximumNumberOfWebhooksReached = 30_007,

        #[doc(alias = "30008")]
        MaximumNumberOfEmojisReached = 30_008,

        #[doc(alias = "30010")]
        MaximumNumberOfReactionsReached = 30_010,

        #[doc(alias = "MaximumNumberOfGroupDMsReached")]
        #[doc(alias = "30011")]
        MaximumNumberOfGroupDmsReached = 30_011,

        #[doc(alias = "30013")]
        MaximumNumberOfGuildChannelsReached = 30_013,

        #[doc(alias = "30015")]
        MaximumNumberOfAttachmentsInAMessageReached = 30_015,

        #[doc(alias = "30016")]
        MaximumNumberOfInvitesReached = 30_016,

        #[doc(alias = "30018")]
        MaximumNumberOfAnimatedEmojisReached = 30_018,

        #[doc(alias = "30019")]
        MaximumNumberOfServerMembersReached = 30_019,

        #[doc(alias = "30030")]
        MaximumNumberOfServerCategoriesReached = 30_030,

        #[doc(alias = "30031")]
        GuildAlreadyHasTemplate = 30_031,

        #[doc(alias = "30032")]
        MaximumNumberOfApplicationCommandsReached = 30_032,

        #[doc(alias = "30033")]
        MaximumThreadParticipantsReached = 30_033,

        #[doc(alias = "30034")]
        MaximumDailyApplicationCommandCreatesReached = 30_034,

        #[doc(alias = "30035")]
        MaximumNumberOfNonGuildMemberBansHasBeenExceeded = 30_035,

        #[doc(alias = "30037")]
        MaximumNumberOfBanFetchesHasBeenReached = 30_037,

        #[doc(alias = "30038")]
        MaximumNumberOfUncompletedGuildScheduledEventsReached = 30_038,

        #[doc(alias = "30039")]
        MaximumNumberOfStickersReached = 30_039,

        #[doc(alias = "30040")]
        MaximumNumberOfPruneRequestsHasBeenReached = 30_040,

        #[doc(alias = "30042")]
        MaximumNumberOfGuildWidgetSettingsUpdatesHasBeenReached = 30_042,

        #[doc(alias = "30045")]
        MaximumNumberOfSoundboardSoundsReached = 30_045,

        #[doc(alias = "30046")]
        MaximumNumberOfEditsToMessagesOlderThanOneHourReached = 30_046,

        #[doc(alias = "30047")]
        MaximumNumberOfPinnedThreadsInForumHasBeenReached = 30_047,

        #[doc(alias = "30048")]
        MaximumNumberOfTagsInForumHasBeenReached = 30_048,

        #[doc(alias = "30052")]
        BitrateIsTooHighForChannelOfThisType = 30_052,

        #[doc(alias = "30056")]
        MaximumNumberOfPremiumEmojisReached = 30_056,

        #[doc(alias = "30058")]
        MaximumNumberOfWebhooksPerGuildReached = 30_058,

        #[doc(alias = "30060")]
        MaximumNumberOfChannelPermissionOverwritesReached = 30_060,

        #[doc(alias = "30061")]
        TheChannelsForThisGuildAreTooLarge = 30_061,

        #[doc(alias = "30084")]
        InviteTargetUsersLimitReached = 30_084,

        #[doc(alias = "40001")]
        Unauthorized = 40_001,

        #[doc(alias = "40002")]
        VerifyYourAccount = 40_002,

        #[doc(alias = "40003")]
        OpeningDirectMessagesTooFast = 40_003,

        #[doc(alias = "40004")]
        SendMessagesHasBeenTemporarilyDisabled = 40_004,

        #[doc(alias = "40005")]
        RequestEntityTooLarge = 40_005,

        #[doc(alias = "40006")]
        FeatureTemporarilyDisabledServerSide = 40_006,

        #[doc(alias = "40007")]
        UserBannedFromThisGuild = 40_007,

        #[doc(alias = "40009")]
        OnlyOneChannelCanHaveAParentIdModifiedAtATime = 40_009,

        #[doc(alias = "40012")]
        ConnectionHasBeenRevoked = 40_012,

        #[doc(alias = "OnlyConsumableSKUsCanBeConsumed")]
        #[doc(alias = "40018")]
        OnlyConsumableSkusCanBeConsumed = 40_018,

        #[doc(alias = "40019")]
        YouCanOnlyDeleteSandboxEntitlements = 40_019,

        #[doc(alias = "40032")]
        TargetUserIsNotConnectedToVoice = 40_032,

        #[doc(alias = "40033")]
        ThisMessageWasAlreadyCrossposted = 40_033,

        #[doc(alias = "40041")]
        ApplicationCommandWithThatNameAlreadyExists = 40_041,

        #[doc(alias = "40043")]
        ApplicationInteractionFailedToSend = 40_043,

        #[doc(alias = "40058")]
        CannotSendAMessageInAForumChannel = 40_058,

        #[doc(alias = "40060")]
        InteractionHasAlreadyBeenAcknowledged = 40_060,

        #[doc(alias = "40061")]
        TagNamesMustBeUnique = 40_061,

        #[doc(alias = "40062")]
        ServiceResourceIsBeingRateLimited = 40_062,

        #[doc(alias = "40066")]
        ThereAreNoTagsAvailableThatCanBeSetByNonModerators = 40_066,

        #[doc(alias = "40067")]
        TagRequiredToCreateAForumPostInThisChannel = 40_067,

        #[doc(alias = "40074")]
        AnEntitlementHasAlreadyBeenGrantedForThisResource = 40_074,

        #[doc(alias = "40094")]
        ThisInteractionHasHitTheMaximumNumberOfFollowUpMessages = 40_094,

        #[doc(alias = "40114")]
        InviteTargetUsersAlreadyBeingProcessed = 40_114,

        #[doc(alias = "40115")]
        InviteTargetUsersFileNotProcessed = 40_115,


        #[doc(alias = "40333")]
        CloudflareIsBlockingYourRequest = 40_333,

        #[doc(alias = "50001")]
        MissingAccess = 50_001,

        #[doc(alias = "50002")]
        InvalidAccountType = 50_002,

        #[doc(alias = "CannotExecuteActionOnDMChannel")]
        #[doc(alias = "50003")]
        CannotExecuteActionOnDmChannel = 50_003,

        #[doc(alias = "50004")]
        GuildWidgetDisabled = 50_004,

        #[doc(alias = "50005")]
        CannotEditMessageAuthoredByAnotherUser = 50_005,

        #[doc(alias = "50006")]
        CannotSendAnEmptyMessage = 50_006,

        #[doc(alias = "50007")]
        CannotSendMessagesToThisUser = 50_007,

        #[doc(alias = "50008")]
        CannotSendMessagesInNonTextChannel = 50_008,

        #[doc(alias = "50009")]
        ChannelVerificationLevelTooHighForYouToGainAccess = 50_009,

        #[doc(alias = "50010")]
        OAuth2ApplicationDoesNotHaveBot = 50_010,

        #[doc(alias = "50011")]
        OAuth2ApplicationLimitReached = 50_011,

        #[doc(alias = "50012")]
        InvalidOAuth2State = 50_012,

        #[doc(alias = "50013")]
        MissingPermissions = 50_013,

        #[doc(alias = "50014")]
        InvalidToken = 50_014,

        #[doc(alias = "50015")]
        NoteWasTooLong = 50_015,

        #[doc(alias = "50016")]
        ProvidedTooFewOrTooManyMessagesToDelete = 50_016,

        #[doc(alias = "InvalidMFALevel")]
        #[doc(alias = "50017")]
        InvalidMfaLevel = 50_017,

        #[doc(alias = "50019")]
        MessageCanOnlyBePinnedInTheChannelItWasSentIn = 50_019,

        #[doc(alias = "50020")]
        InviteCodeInvalidOrTaken = 50_020,

        #[doc(alias = "50021")]
        CannotExecuteActionOnSystemMessage = 50_021,

        #[doc(alias = "50024")]
        CannotExecuteActionOnThisChannelType = 50_024,

        #[doc(alias = "50025")]
        InvalidOAuth2AccessToken = 50_025,

        #[doc(alias = "50026")]
        MissingRequiredOAuth2Scope = 50_026,

        #[doc(alias = "50027")]
        InvalidWebhookToken = 50_027,

        #[doc(alias = "50028")]
        InvalidRole = 50_028,

        #[doc(alias = "50033")]
        InvalidRecipients = 50_033,

        #[doc(alias = "50034")]
        OneOfTheMessagesProvidedWasTooOldForBulkDelete = 50_034,


        #[doc(alias = "50035")]
        InvalidFormBodyOrContentType = 50_035,

        #[doc(alias = "50036")]
        InviteAcceptedToGuildWithoutTheBotBeingIn = 50_036,

        #[doc(alias = "50039")]
        InvalidActivityAction = 50_039,

        #[doc(alias = "InvalidAPIVersion")]
        #[doc(alias = "50041")]
        InvalidApiVersion = 50_041,

        #[doc(alias = "50045")]
        FileUploadedExceedsMaximumSize = 50_045,

        #[doc(alias = "50046")]
        InvalidFileUploaded = 50_046,

        #[doc(alias = "50054")]
        CannotSelfRedeemThisGift = 50_054,

        #[doc(alias = "50055")]
        InvalidGuild = 50_055,

        #[doc(alias = "InvalidSKU")]
        #[doc(alias = "50057")]
        InvalidSku = 50_057,

        #[doc(alias = "50067")]
        InvalidRequestOrigin = 50_067,

        #[doc(alias = "50068")]
        InvalidMessageType = 50_068,

        #[doc(alias = "50070")]
        PaymentSourceRequiredToRedeemGift = 50_070,

        #[doc(alias = "50073")]
        CannotModifyASystemWebhook = 50_073,

        #[doc(alias = "50074")]
        CannotDeleteChannelRequiredForCommunityGuilds = 50_074,

        #[doc(alias = "50080")]
        CannotEditStickersWithinMessage = 50_080,

        #[doc(alias = "50081")]
        InvalidStickerSent = 50_081,


        #[doc(alias = "50083")]
        InvalidActionOnArchivedThread = 50_083,

        #[doc(alias = "50084")]
        InvalidThreadNotificationSettings = 50_084,

        #[doc(alias = "50085")]
        ParameterEarlierThanCreation = 50_085,

        #[doc(alias = "50086")]
        CommunityServerChannelsMustBeTextChannels = 50_086,

        #[doc(alias = "50091")]
        TheEntityTypeOfTheEventIsDifferentFromTheEntityYouAreTryingToStartTheEventFor = 50_091,

        #[doc(alias = "50095")]
        ServerNotAvailableInYourLocation = 50_095,

        #[doc(alias = "50097")]
        ServerNeedsMonetizationEnabledToPerformThisAction = 50_097,

        #[doc(alias = "50101")]
        ServerNeedsMoreBoostsToPerformThisAction = 50_101,

        #[doc(alias = "RequestBodyContainsInvalidJSON")]
        #[doc(alias = "50109")]
        RequestBodyContainsInvalidJson = 50_109,

        #[doc(alias = "50110")]
        ProvidedFileIsInvalid = 50_110,

        #[doc(alias = "50123")]
        ProvidedFileTypeIsInvalid = 50_123,

        #[doc(alias = "50124")]
        ProvidedFileDurationExceedsMaximumLength = 50_124,

        #[doc(alias = "50131")]
        OwnerCannotBePendingMember = 50_131,

        #[doc(alias = "50132")]
        OwnershipCannotBeMovedToABotUser = 50_132,

        #[doc(alias = "50138")]
        FailedToResizeAssetBelowTheMaximumSize = 50_138,

        #[doc(alias = "50144")]
        CannotMixSubscriptionAndNonSubscriptionRolesForAnEmoji = 50_144,

        #[doc(alias = "50145")]
        CannotConvertBetweenPremiumEmojiAndNormalEmoji = 50_145,

        #[doc(alias = "50146")]
        UploadedFileNotFound = 50_146,

        #[doc(alias = "50151")]
        SpecifiedEmojiIsInvalid = 50_151,

        #[doc(alias = "50159")]
        VoiceMessagesDoNotSupportAdditionalContent = 50_159,

        #[doc(alias = "50160")]
        VoiceMessagesMustHaveASingleAudioAttachment = 50_160,

        #[doc(alias = "50161")]
        VoiceMessagesMustHaveSupportingMetadata = 50_161,

        #[doc(alias = "50162")]
        VoiceMessagesCannotBeEdited = 50_162,

        #[doc(alias = "50163")]
        CannotDeleteGuildSubscriptionIntegration = 50_163,


        #[doc(alias = "50167")]
        CannotSendVoiceEffectWhenUserIsServerMutedDeafenedOrSuppressed = 50_167,

        #[doc(alias = "50173")]
        YouCannotSendVoiceMessagesInThisChannel = 50_173,

        #[doc(alias = "50178")]
        TheUserAccountMustFirstBeVerified = 50_178,

        #[doc(alias = "50192")]
        ProvidedFileDoesNotHaveAValidDuration = 50_192,

        #[doc(alias = "50278")]
        CannotSendMessagesToThisUserDueToHavingNoMutualGuilds = 50_278,

        #[doc(alias = "50600")]
        YouDoNotHavePermissionToSendThisSticker = 50_600,

        #[doc(alias = "60003")]
        TwoFactorAuthenticationIsRequired = 60_003,

        #[doc(alias = "80004")]
        NoUsersWithDiscordTagExist = 80_004,

        #[doc(alias = "90001")]
        ReactionWasBlocked = 90_001,

        #[doc(alias = "90002")]
        UserCannotUseBurstReactions = 90_002,

        #[doc(alias = "110000")]
        IndexNotYetAvailable = 110_000,

        #[doc(alias = "110001")]
        ApplicationNotYetAvailable = 110_001,

        #[doc(alias = "APIResourceOverloaded")]
        #[doc(alias = "130000")]
        ApiResourceOverloaded = 130_000,

        #[doc(alias = "150006")]
        TheStageIsAlreadyOpen = 150_006,

        #[doc(alias = "160002")]
        CannotReplyWithoutPermissionToReadMessageHistory = 160_002,

        #[doc(alias = "160004")]
        ThreadAlreadyCreatedForMessage = 160_004,

        #[doc(alias = "160005")]
        ThreadLocked = 160_005,

        #[doc(alias = "160006")]
        MaximumActiveThreads = 160_006,

        #[doc(alias = "160007")]
        MaximumActiveAnnouncementThreads = 160_007,

        #[doc(alias = "160009")]
        CannotReferenceAMessageWithoutPermissionToReadMessageHistory = 160_009,

        #[doc(alias = "NSFWChannelMessageReferenceNotAllowed")]
        #[doc(alias = "160010")]
        NsfwChannelMessageReferenceNotAllowed = 160_010,

        #[doc(alias = "160014")]
        CannotForwardMessageWithUnreadableContent = 160_014,

        #[doc(alias = "InvalidJSONForUploadedLottieFile")]
        #[doc(alias = "170001")]
        InvalidJsonForUploadedLottieFile = 170_001,

        #[doc(alias = "170002")]
        UploadedLottiesCannotContainRasterizedImages = 170_002,

        #[doc(alias = "170003")]
        StickerMaximumFramerateExceeded = 170_003,

        #[doc(alias = "170004")]
        StickerFrameCountExceedsMaximumOf1000Frames = 170_004,

        #[doc(alias = "170005")]
        LottieAnimationMaximumDimensionsExceeded = 170_005,

        #[doc(alias = "170006")]
        StickerFramerateIsTooSmallOrTooLarge = 170_006,

        #[doc(alias = "170007")]
        StickerAnimationDurationExceedsMaximumOf5Seconds = 170_007,

        #[doc(alias = "180000")]
        CannotUpdateAFinishedEvent = 180_000,

        #[doc(alias = "180002")]
        FailedToCreateStageNeededForStageEvent = 180_002,

        #[doc(alias = "200000")]
        MessageWasBlockedByAutomaticModeration = 200_000,

        #[doc(alias = "200001")]
        TitleWasBlockedByAutomaticModeration = 200_001,

        #[doc(alias = "220001")]
        WebhooksPostedToForumChannelsMustHaveAThreadNameOrThreadId = 220_001,


        #[doc(alias = "220002")]
        WebhooksPostedToForumChannelsCannotHaveBothAThreadNameAndThreadId = 220_002,

        #[doc(alias = "220003")]
        WebhooksCanOnlyCreateThreadsInForumChannels = 220_003,

        #[doc(alias = "220004")]
        WebhookServicesCannotBeUsedInForumChannels = 220_004,

        #[doc(alias = "240000")]
        MessageBlockedByHarmfulLinksFilter = 240_000,

        #[doc(alias = "340015")]
        AccessToJoiningNewServersHasBeenLimitedForTheUser = 340_015,

        #[doc(alias = "350000")]
        CannotEnableOnboardingRequirementsAreNotMet = 350_000,

        #[doc(alias = "350001")]
        CannotUpdateOnboardingWhileBelowRequirements = 350_001,

        #[doc(alias = "400001")]
        AccessToFileUploadsHasBeenLimitedForThisGuild = 400_001,

        #[doc(alias = "500000")]
        FailedToBanUsers = 500_000,

        #[doc(alias = "520000")]
        PollVotingBlocked = 520_000,

        #[doc(alias = "520001")]
        PollExpired = 520_001,

        #[doc(alias = "520002")]
        InvalidChannelTypeForPollCreation = 520_002,

        #[doc(alias = "520003")]
        CannotEditAPollMessage = 520_003,

        #[doc(alias = "520004")]
        CannotUseAnEmojiIncludedWithThePoll = 520_004,

        #[doc(alias = "520006")]
        CannotExpireANonPollMessage = 520_006,

        #[doc(alias = "530000")]
        ProvisionalAccountsPermissionNotGranted = 530_000,

        #[doc(alias = "IdTokenJWTExpired")]
        #[doc(alias = "530001")]
        IdTokenJwtExpired = 530_001,

        #[doc(alias = "IdTokenJWTIssuerMismatch")]
        #[doc(alias = "530002")]
        IdTokenJwtIssuerMismatch = 530_002,


        #[doc(alias = "IdTokenJWTAudienceMismatch")]
        #[doc(alias = "530003")]
        IdTokenJwtAudienceMismatch = 530_003,


        #[doc(alias = "IdTokenJWTIssuedTooLongAgo")]
        #[doc(alias = "530004")]
        IdTokenJwtIssuedTooLongAgo = 530_004,

        #[doc(alias = "530006")]
        FailedToGenerateUniqueUsername = 530_006,

        #[doc(alias = "530007")]
        InvalidClientSecret = 530_007,
    }
}
