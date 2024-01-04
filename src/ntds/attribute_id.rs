use strum::{EnumString, IntoStaticStr};

use serde::{Serialize, Deserialize};

#[derive(IntoStaticStr, EnumString, Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd, Serialize, Deserialize)]
#[strum(use_phf)]
pub enum NtdsAttributeId {
    #[strum(serialize = "ATTq589983", to_string = "AttAccountExpires")]
    AttAccountExpires = 0x9009f,
    #[strum(serialize = "ATTm591131", to_string = "AttAccountNameHistory")]
    AttAccountNameHistory = 0x9051b,
    #[strum(serialize = "ATTq590584", to_string = "AttAcsAggregateTokenRatePerUser")]
    AttAcsAggregateTokenRatePerUser = 0x902f8,
    #[strum(serialize = "ATTq590590", to_string = "AttAcsAllocableRsvpBandwidth")]
    AttAcsAllocableRsvpBandwidth = 0x902fe,
    #[strum(serialize = "ATTj590603", to_string = "AttAcsCacheTimeout")]
    AttAcsCacheTimeout = 0x9030b,
    #[strum(serialize = "ATTj590581", to_string = "AttAcsDirection")]
    AttAcsDirection = 0x902f5,
    #[strum(serialize = "ATTj590602", to_string = "AttAcsDsbmDeadtime")]
    AttAcsDsbmDeadtime = 0x9030a,
    #[strum(serialize = "ATTj590600", to_string = "AttAcsDsbmPriority")]
    AttAcsDsbmPriority = 0x90308,
    #[strum(serialize = "ATTj590601", to_string = "AttAcsDsbmRefresh")]
    AttAcsDsbmRefresh = 0x90309,
    #[strum(serialize = "ATTi590594", to_string = "AttAcsEnableAcsService")]
    AttAcsEnableAcsService = 0x90302,
    #[strum(serialize = "ATTi590723", to_string = "AttAcsEnableRsvpAccounting")]
    AttAcsEnableRsvpAccounting = 0x90383,
    #[strum(serialize = "ATTi590592", to_string = "AttAcsEnableRsvpMessageLogging")]
    AttAcsEnableRsvpMessageLogging = 0x90300,
    #[strum(serialize = "ATTj590593", to_string = "AttAcsEventLogLevel")]
    AttAcsEventLogLevel = 0x90301,
    #[strum(serialize = "ATTm590608", to_string = "AttAcsIdentityName")]
    AttAcsIdentityName = 0x90310,
    #[strum(serialize = "ATTq590721", to_string = "AttAcsMaxAggregatePeakRatePerUser")]
    AttAcsMaxAggregatePeakRatePerUser = 0x90381,
    #[strum(serialize = "ATTj590585", to_string = "AttAcsMaxDurationPerFlow")]
    AttAcsMaxDurationPerFlow = 0x902f9,
    #[strum(serialize = "ATTj590725", to_string = "AttAcsMaxNoOfAccountFiles")]
    AttAcsMaxNoOfAccountFiles = 0x90385,
    #[strum(serialize = "ATTj590598", to_string = "AttAcsMaxNoOfLogFiles")]
    AttAcsMaxNoOfLogFiles = 0x90306,
    #[strum(serialize = "ATTq590591", to_string = "AttAcsMaxPeakBandwidth")]
    AttAcsMaxPeakBandwidth = 0x902ff,
    #[strum(serialize = "ATTq590583", to_string = "AttAcsMaxPeakBandwidthPerFlow")]
    AttAcsMaxPeakBandwidthPerFlow = 0x902f7,
    #[strum(serialize = "ATTj590726", to_string = "AttAcsMaxSizeOfRsvpAccountFile")]
    AttAcsMaxSizeOfRsvpAccountFile = 0x90386,
    #[strum(serialize = "ATTj590599", to_string = "AttAcsMaxSizeOfRsvpLogFile")]
    AttAcsMaxSizeOfRsvpLogFile = 0x90307,
    #[strum(serialize = "ATTq591137", to_string = "AttAcsMaxTokenBucketPerFlow")]
    AttAcsMaxTokenBucketPerFlow = 0x90521,
    #[strum(serialize = "ATTq590582", to_string = "AttAcsMaxTokenRatePerFlow")]
    AttAcsMaxTokenRatePerFlow = 0x902f6,
    #[strum(serialize = "ATTq591138", to_string = "AttAcsMaximumSduSize")]
    AttAcsMaximumSduSize = 0x90522,
    #[strum(serialize = "ATTq591141", to_string = "AttAcsMinimumDelayVariation")]
    AttAcsMinimumDelayVariation = 0x90525,
    #[strum(serialize = "ATTq591140", to_string = "AttAcsMinimumLatency")]
    AttAcsMinimumLatency = 0x90524,
    #[strum(serialize = "ATTq591139", to_string = "AttAcsMinimumPolicedSize")]
    AttAcsMinimumPolicedSize = 0x90523,
    #[strum(serialize = "ATTq591144", to_string = "AttAcsNonReservedMaxSduSize")]
    AttAcsNonReservedMaxSduSize = 0x90528,
    #[strum(serialize = "ATTq591145", to_string = "AttAcsNonReservedMinPolicedSize")]
    AttAcsNonReservedMinPolicedSize = 0x90529,
    #[strum(serialize = "ATTq591142", to_string = "AttAcsNonReservedPeakRate")]
    AttAcsNonReservedPeakRate = 0x90526,
    #[strum(serialize = "ATTq591143", to_string = "AttAcsNonReservedTokenSize")]
    AttAcsNonReservedTokenSize = 0x90527,
    #[strum(serialize = "ATTq590604", to_string = "AttAcsNonReservedTxLimit")]
    AttAcsNonReservedTxLimit = 0x9030c,
    #[strum(serialize = "ATTq590722", to_string = "AttAcsNonReservedTxSize")]
    AttAcsNonReservedTxSize = 0x90382,
    #[strum(serialize = "ATTq590589", to_string = "AttAcsPermissionBits")]
    AttAcsPermissionBits = 0x902fd,
    #[strum(serialize = "ATTm590596", to_string = "AttAcsPolicyName")]
    AttAcsPolicyName = 0x90304,
    #[strum(serialize = "ATTj590588", to_string = "AttAcsPriority")]
    AttAcsPriority = 0x902fc,
    #[strum(serialize = "ATTm590724", to_string = "AttAcsRsvpAccountFilesLocation")]
    AttAcsRsvpAccountFilesLocation = 0x90384,
    #[strum(serialize = "ATTm590597", to_string = "AttAcsRsvpLogFilesLocation")]
    AttAcsRsvpLogFilesLocation = 0x90305,
    #[strum(serialize = "ATTj590586", to_string = "AttAcsServiceType")]
    AttAcsServiceType = 0x902fa,
    #[strum(serialize = "ATTm590580", to_string = "AttAcsTimeOfDay")]
    AttAcsTimeOfDay = 0x902f4,
    #[strum(serialize = "ATTj590587", to_string = "AttAcsTotalNoOfFlows")]
    AttAcsTotalNoOfFlows = 0x902fb,
    #[strum(serialize = "ATTm591136", to_string = "AttAcsServerList")]
    AttAcsServerList = 0x90520,
    #[strum(serialize = "ATTm590089", to_string = "AttAdditionalInformation")]
    AttAdditionalInformation = 0x90109,
    #[strum(serialize = "ATTm590713", to_string = "AttAdditionalTrustedServiceNames")]
    AttAdditionalTrustedServiceNames = 0x90379,
    #[strum(serialize = "ATTm131328", to_string = "AttAddress")]
    AttAddress = 0x20100,
    #[strum(serialize = "ATTb591068", to_string = "AttAddressBookRoots")]
    AttAddressBookRoots = 0x904dc,
    #[strum(serialize = "ATTk131396", to_string = "AttAddressEntryDisplayTable")]
    AttAddressEntryDisplayTable = 0x20144,
    #[strum(serialize = "ATTk131472", to_string = "AttAddressEntryDisplayTableMsdos")]
    AttAddressEntryDisplayTableMsdos = 0x20190,
    #[strum(serialize = "ATTm131689", to_string = "AttAddressHome")]
    AttAddressHome = 0x20269,
    #[strum(serialize = "ATTk131327", to_string = "AttAddressSyntax")]
    AttAddressSyntax = 0x200ff,
    #[strum(serialize = "ATTe131422", to_string = "AttAddressType")]
    AttAddressType = 0x2015e,
    #[strum(serialize = "ATTm590438", to_string = "AttAdminContextMenu")]
    AttAdminContextMenu = 0x90266,
    #[strum(serialize = "ATTj589974", to_string = "AttAdminCount")]
    AttAdminCount = 0x90096,
    #[strum(serialize = "ATTm131298", to_string = "AttAdminDescription")]
    AttAdminDescription = 0x200e2,
    #[strum(serialize = "ATTm131266", to_string = "AttAdminDisplayName")]
    AttAdminDisplayName = 0x200c2,
    #[strum(serialize = "ATTm591514", to_string = "AttAdminMultiselectPropertyPages")]
    AttAdminMultiselectPropertyPages = 0x9069a,
    #[strum(serialize = "ATTm590386", to_string = "AttAdminPropertyPages")]
    AttAdminPropertyPages = 0x90232,
    #[strum(serialize = "ATTc590737", to_string = "AttAllowedAttributes")]
    AttAllowedAttributes = 0x90391,
    #[strum(serialize = "ATTc590738", to_string = "AttAllowedAttributesEffective")]
    AttAllowedAttributesEffective = 0x90392,
    #[strum(serialize = "ATTc590735", to_string = "AttAllowedChildClasses")]
    AttAllowedChildClasses = 0x9038f,
    #[strum(serialize = "ATTc590736", to_string = "AttAllowedChildClassesEffective")]
    AttAllowedChildClassesEffective = 0x90390,
    #[strum(serialize = "ATTm590691", to_string = "AttAltSecurityIdentities")]
    AttAltSecurityIdentities = 0x90363,
    #[strum(serialize = "ATTm591032", to_string = "AttAnr")]
    AttAnr = 0x904b8,
    #[strum(serialize = "ATTj590672", to_string = "AttAppSchemaVersion")]
    AttAppSchemaVersion = 0x90350,
    #[strum(serialize = "ATTm590042", to_string = "AttApplicationName")]
    AttApplicationName = 0x900da,
    #[strum(serialize = "ATTm590165", to_string = "AttAppliesTo")]
    AttAppliesTo = 0x90155,
    #[strum(serialize = "ATTm590107", to_string = "AttAssetNumber")]
    AttAssetNumber = 0x9011b,
    #[strum(serialize = "ATTb590476", to_string = "AttAssistant")]
    AttAssistant = 0x9028c,
    #[strum(serialize = "ATTk591037", to_string = "AttAssocNtAccount")]
    AttAssocNtAccount = 0x904bd,
    #[strum(serialize = "ATTk58", to_string = "AttAttributecertificateattribute")]
    AttAttributecertificateattribute = 0x3a,
    #[strum(serialize = "ATTm590572", to_string = "AttAttributeDisplayNames")]
    AttAttributeDisplayNames = 0x902ec,
    #[strum(serialize = "ATTc131102", to_string = "AttAttributeId")]
    AttAttributeId = 0x2001e,
    #[strum(serialize = "ATTk589973", to_string = "AttAttributeSecurityGuid")]
    AttAttributeSecurityGuid = 0x90095,
    #[strum(serialize = "ATTc131104", to_string = "AttAttributeSyntax")]
    AttAttributeSyntax = 0x20020,
    #[strum(serialize = "ATTm1572869", to_string = "AttAttributeTypes")]
    AttAttributeTypes = 0x180005,
    #[strum(serialize = "ATTk1376311", to_string = "AttAudio")]
    AttAudio = 0x150037,
    #[strum(serialize = "ATTk590026", to_string = "AttAuditingPolicy")]
    AttAuditingPolicy = 0x900ca,
    #[strum(serialize = "ATTj589835", to_string = "AttAuthenticationOptions")]
    AttAuthenticationOptions = 0x9000b,
    #[strum(serialize = "ATTk38", to_string = "AttAuthorityRevocationList")]
    AttAuthorityRevocationList = 0x26,
    #[strum(serialize = "ATTc131423", to_string = "AttAuxiliaryClass")]
    AttAuxiliaryClass = 0x2015f,
    #[strum(serialize = "ATTq589873", to_string = "AttBadPasswordTime")]
    AttBadPasswordTime = 0x90031,
    #[strum(serialize = "ATTj589836", to_string = "AttBadPwdCount")]
    AttBadPwdCount = 0x9000c,
    #[strum(serialize = "ATTk590156", to_string = "AttBirthLocation")]
    AttBirthLocation = 0x9014c,
    #[strum(serialize = "ATTb590644", to_string = "AttBridgeheadServerListBl")]
    AttBridgeheadServerListBl = 0x90334,
    #[strum(serialize = "ATTb590643", to_string = "AttBridgeheadTransportList")]
    AttBridgeheadTransportList = 0x90333,
    #[strum(serialize = "ATTq589837", to_string = "AttBuiltinCreationTime")]
    AttBuiltinCreationTime = 0x9000d,
    #[strum(serialize = "ATTq589838", to_string = "AttBuiltinModifiedCount")]
    AttBuiltinModifiedCount = 0x9000e,
    #[strum(serialize = "ATTm15", to_string = "AttBusinessCategory")]
    AttBusinessCategory = 0xf,
    #[strum(serialize = "ATTj590108", to_string = "AttBytesPerMinute")]
    AttBytesPerMinute = 0x9011c,
    #[strum(serialize = "ATTk37", to_string = "AttCaCertificate")]
    AttCaCertificate = 0x25,
    #[strum(serialize = "ATTm590521", to_string = "AttCaCertificateDn")]
    AttCaCertificateDn = 0x902b9,
    #[strum(serialize = "ATTm590511", to_string = "AttCaConnect")]
    AttCaConnect = 0x902af,
    #[strum(serialize = "ATTm590514", to_string = "AttCaUsages")]
    AttCaUsages = 0x902b2,
    #[strum(serialize = "ATTm590512", to_string = "AttCaWebUrl")]
    AttCaWebUrl = 0x902b0,
    #[strum(serialize = "ATTm590639", to_string = "AttCanUpgradeScript")]
    AttCanUpgradeScript = 0x9032f,
    #[strum(serialize = "ATTm590740", to_string = "AttCanonicalName")]
    AttCanonicalName = 0x90394,
    #[strum(serialize = "ATTm1441793", to_string = "AttCarlicense")]
    AttCarlicense = 0x160001,
    #[strum(serialize = "ATTm590499", to_string = "AttCatalogs")]
    AttCatalogs = 0x902a3,
    #[strum(serialize = "ATTm590496", to_string = "AttCategories")]
    AttCategories = 0x902a0,
    #[strum(serialize = "ATTk590146", to_string = "AttCategoryId")]
    AttCategoryId = 0x90142,
    #[strum(serialize = "ATTb590508", to_string = "AttCertificateAuthorityObject")]
    AttCertificateAuthorityObject = 0x902ac,
    #[strum(serialize = "ATTk39", to_string = "AttCertificateRevocationList")]
    AttCertificateRevocationList = 0x27,
    #[strum(serialize = "ATTm590647", to_string = "AttCertificateTemplates")]
    AttCertificateTemplates = 0x90337,
    #[strum(serialize = "ATTm590434", to_string = "AttClassDisplayName")]
    AttClassDisplayName = 0x90262,
    #[strum(serialize = "ATTj589840", to_string = "AttCodePage")]
    AttCodePage = 0x90010,
    #[strum(serialize = "ATTm589843", to_string = "AttComClassid")]
    AttComClassid = 0x90013,
    #[strum(serialize = "ATTm590073", to_string = "AttComClsid")]
    AttComClsid = 0x900f9,
    #[strum(serialize = "ATTm589844", to_string = "AttComInterfaceid")]
    AttComInterfaceid = 0x90014,
    #[strum(serialize = "ATTm590077", to_string = "AttComOtherProgId")]
    AttComOtherProgId = 0x900fd,
    #[strum(serialize = "ATTm589845", to_string = "AttComProgid")]
    AttComProgid = 0x90015,
    #[strum(serialize = "ATTm590075", to_string = "AttComTreatAsClassId")]
    AttComTreatAsClassId = 0x900fb,
    #[strum(serialize = "ATTm590078", to_string = "AttComTypelibId")]
    AttComTypelibId = 0x900fe,
    #[strum(serialize = "ATTm590074", to_string = "AttComUniqueLibid")]
    AttComUniqueLibid = 0x900fa,
    #[strum(serialize = "ATTm131153", to_string = "AttComment")]
    AttComment = 0x20051,
    #[strum(serialize = "ATTm3", to_string = "AttCommonName")]
    AttCommonName = 0x3,
    #[strum(serialize = "ATTm131218", to_string = "AttCompany")]
    AttCompany = 0x20092,
    #[strum(serialize = "ATTi589848", to_string = "AttContentIndexingAllowed")]
    AttContentIndexingAllowed = 0x90018,
    #[strum(serialize = "ATTm590323", to_string = "AttContextMenu")]
    AttContextMenu = 0x901f3,
    #[strum(serialize = "ATTk590024", to_string = "AttControlAccessRights")]
    AttControlAccessRights = 0x900c8,
    #[strum(serialize = "ATTj131207", to_string = "AttCost")]
    AttCost = 0x20087,
    #[strum(serialize = "ATTj589849", to_string = "AttCountryCode")]
    AttCountryCode = 0x90019,
    #[strum(serialize = "ATTm6", to_string = "AttCountryName")]
    AttCountryName = 0x6,
    #[strum(serialize = "ATTm590634", to_string = "AttCreateDialog")]
    AttCreateDialog = 0x9032a,
    #[strum(serialize = "ATTl1638401", to_string = "AttCreateTimeStamp")]
    AttCreateTimeStamp = 0x190001,
    #[strum(serialize = "ATTm590636", to_string = "AttCreateWizardExt")]
    AttCreateWizardExt = 0x9032c,
    #[strum(serialize = "ATTq589850", to_string = "AttCreationTime")]
    AttCreationTime = 0x9001a,
    #[strum(serialize = "ATTm590322", to_string = "AttCreationWizard")]
    AttCreationWizard = 0x901f2,
    #[strum(serialize = "ATTm590503", to_string = "AttCreator")]
    AttCreator = 0x902a7,
    #[strum(serialize = "ATTb590513", to_string = "AttCrlObject")]
    AttCrlObject = 0x902b1,
    #[strum(serialize = "ATTk590507", to_string = "AttCrlPartitionedRevocationList")]
    AttCrlPartitionedRevocationList = 0x902ab,
    #[strum(serialize = "ATTk40", to_string = "AttCrossCertificatePair")]
    AttCrossCertificatePair = 0x28,
    #[strum(serialize = "ATTk590161", to_string = "AttCurrMachineId")]
    AttCurrMachineId = 0x90151,
    #[strum(serialize = "ATTk590159", to_string = "AttCurrentLocation")]
    AttCurrentLocation = 0x9014f,
    #[strum(serialize = "ATTb590520", to_string = "AttCurrentParentCa")]
    AttCurrentParentCa = 0x902b8,
    #[strum(serialize = "ATTk589851", to_string = "AttCurrentValue")]
    AttCurrentValue = 0x9001b,
    #[strum(serialize = "ATTk589879", to_string = "AttDbcsPwd")]
    AttDbcsPwd = 0x90037,
    #[strum(serialize = "ATTb590037", to_string = "AttDefaultClassStore")]
    AttDefaultClassStore = 0x900d5,
    #[strum(serialize = "ATTb590304", to_string = "AttDefaultGroup")]
    AttDefaultGroup = 0x901e0,
    #[strum(serialize = "ATTi590342", to_string = "AttDefaultHidingValue")]
    AttDefaultHidingValue = 0x90206,
    #[strum(serialize = "ATTb589881", to_string = "AttDefaultLocalPolicyObject")]
    AttDefaultLocalPolicyObject = 0x90039,
    #[strum(serialize = "ATTb590607", to_string = "AttDefaultObjectCategory")]
    AttDefaultObjectCategory = 0x9030f,
    #[strum(serialize = "ATTj590056", to_string = "AttDefaultPriority")]
    AttDefaultPriority = 0x900e8,
    #[strum(serialize = "ATTm590048", to_string = "AttDefaultSecurityDescriptor")]
    AttDefaultSecurityDescriptor = 0x900e0,
    #[strum(serialize = "ATTk53", to_string = "AttDeltaRevocationList")]
    AttDeltaRevocationList = 0x35,
    #[strum(serialize = "ATTm131213", to_string = "AttDepartment")]
    AttDepartment = 0x2008d,
    #[strum(serialize = "ATTm1441794", to_string = "AttDepartmentnumber")]
    AttDepartmentnumber = 0x160002,
    #[strum(serialize = "ATTm13", to_string = "AttDescription")]
    AttDescription = 0xd,
    #[strum(serialize = "ATTm590170", to_string = "AttDesktopProfile")]
    AttDesktopProfile = 0x9015a,
    #[strum(serialize = "ATTf27", to_string = "AttDestinationIndicator")]
    AttDestinationIndicator = 0x1b,
    #[strum(serialize = "ATTk590539", to_string = "AttDhcpClasses")]
    AttDhcpClasses = 0x902cb,
    #[strum(serialize = "ATTq590524", to_string = "AttDhcpFlags")]
    AttDhcpFlags = 0x902bc,
    #[strum(serialize = "ATTm590525", to_string = "AttDhcpIdentification")]
    AttDhcpIdentification = 0x902bd,
    #[strum(serialize = "ATTf590530", to_string = "AttDhcpMask")]
    AttDhcpMask = 0x902c2,
    #[strum(serialize = "ATTq590543", to_string = "AttDhcpMaxkey")]
    AttDhcpMaxkey = 0x902cf,
    #[strum(serialize = "ATTm590527", to_string = "AttDhcpObjDescription")]
    AttDhcpObjDescription = 0x902bf,
    #[strum(serialize = "ATTm590526", to_string = "AttDhcpObjName")]
    AttDhcpObjName = 0x902be,
    #[strum(serialize = "ATTk590538", to_string = "AttDhcpOptions")]
    AttDhcpOptions = 0x902ca,
    #[strum(serialize = "ATTk590542", to_string = "AttDhcpProperties")]
    AttDhcpProperties = 0x902ce,
    #[strum(serialize = "ATTf590531", to_string = "AttDhcpRanges")]
    AttDhcpRanges = 0x902c3,
    #[strum(serialize = "ATTf590533", to_string = "AttDhcpReservations")]
    AttDhcpReservations = 0x902c5,
    #[strum(serialize = "ATTf590528", to_string = "AttDhcpServers")]
    AttDhcpServers = 0x902c0,
    #[strum(serialize = "ATTf590532", to_string = "AttDhcpSites")]
    AttDhcpSites = 0x902c4,
    #[strum(serialize = "ATTf590541", to_string = "AttDhcpState")]
    AttDhcpState = 0x902cd,
    #[strum(serialize = "ATTf590529", to_string = "AttDhcpSubnets")]
    AttDhcpSubnets = 0x902c1,
    #[strum(serialize = "ATTj590523", to_string = "AttDhcpType")]
    AttDhcpType = 0x902bb,
    #[strum(serialize = "ATTq590522", to_string = "AttDhcpUniqueKey")]
    AttDhcpUniqueKey = 0x902ba,
    #[strum(serialize = "ATTq590544", to_string = "AttDhcpUpdateTime")]
    AttDhcpUpdateTime = 0x902d0,
    #[strum(serialize = "ATTm131085", to_string = "AttDisplayName")]
    AttDisplayName = 0x2000d,
    #[strum(serialize = "ATTf131425", to_string = "AttDisplayNamePrintable")]
    AttDisplayNamePrintable = 0x20161,
    #[strum(serialize = "ATTm1572866", to_string = "AttDitContentRules")]
    AttDitContentRules = 0x180002,
    #[strum(serialize = "ATTm590085", to_string = "AttDivision")]
    AttDivision = 0x90105,
    #[strum(serialize = "ATTb131108", to_string = "AttDmdLocation")]
    AttDmdLocation = 0x20024,
    #[strum(serialize = "ATTm131670", to_string = "AttDmdName")]
    AttDmdName = 0x20256,
    #[strum(serialize = "ATTb591066", to_string = "AttDnReferenceUpdate")]
    AttDnReferenceUpdate = 0x904da,
    #[strum(serialize = "ATTi590202", to_string = "AttDnsAllowDynamic")]
    AttDnsAllowDynamic = 0x9017a,
    #[strum(serialize = "ATTi590203", to_string = "AttDnsAllowXfr")]
    AttDnsAllowXfr = 0x9017b,
    #[strum(serialize = "ATTm590443", to_string = "AttDnsHostName")]
    AttDnsHostName = 0x9026b,
    #[strum(serialize = "ATTj590205", to_string = "AttDnsNotifySecondaries")]
    AttDnsNotifySecondaries = 0x9017d,
    #[strum(serialize = "ATTk591130", to_string = "AttDnsProperty")]
    AttDnsProperty = 0x9051a,
    #[strum(serialize = "ATTk590206", to_string = "AttDnsRecord")]
    AttDnsRecord = 0x9017e,
    #[strum(serialize = "ATTm589852", to_string = "AttDnsRoot")]
    AttDnsRoot = 0x9001c,
    #[strum(serialize = "ATTj590204", to_string = "AttDnsSecureSecondaries")]
    AttDnsSecureSecondaries = 0x9017c,
    #[strum(serialize = "ATTi591238", to_string = "AttDnsTombstoned")]
    AttDnsTombstoned = 0x90586,
    #[strum(serialize = "ATTb590492", to_string = "AttDomainCertificateAuthorities")]
    AttDomainCertificateAuthorities = 0x9029c,
    #[strum(serialize = "ATTm1376281", to_string = "AttDomainComponent")]
    AttDomainComponent = 0x150019,
    #[strum(serialize = "ATTb590296", to_string = "AttDomainCrossRef")]
    AttDomainCrossRef = 0x901d8,
    #[strum(serialize = "ATTb590510", to_string = "AttDomainId")]
    AttDomainId = 0x902ae,
    #[strum(serialize = "ATTj590579", to_string = "AttDomainIdentifier")]
    AttDomainIdentifier = 0x902f3,
    #[strum(serialize = "ATTb589856", to_string = "AttDomainPolicyObject")]
    AttDomainPolicyObject = 0x90020,
    #[strum(serialize = "ATTb590246", to_string = "AttDomainPolicyReference")]
    AttDomainPolicyReference = 0x901a6,
    #[strum(serialize = "ATTm589982", to_string = "AttDomainReplica")]
    AttDomainReplica = 0x9009e,
    #[strum(serialize = "ATTk590245", to_string = "AttDomainWidePolicy")]
    AttDomainWidePolicy = 0x901a5,
    #[strum(serialize = "ATTm590053", to_string = "AttDriverName")]
    AttDriverName = 0x900e5,
    #[strum(serialize = "ATTj590100", to_string = "AttDriverVersion")]
    AttDriverVersion = 0x90114,
    #[strum(serialize = "ATTl591181", to_string = "AttDsCorePropagationData")]
    AttDsCorePropagationData = 0x9054d,
    #[strum(serialize = "ATTm131284", to_string = "AttDsHeuristics")]
    AttDsHeuristics = 0x200d4,
    #[strum(serialize = "ATTj591168", to_string = "AttDsUiAdminMaximum")]
    AttDsUiAdminMaximum = 0x90540,
    #[strum(serialize = "ATTm591167", to_string = "AttDsUiAdminNotification")]
    AttDsUiAdminNotification = 0x9053f,
    #[strum(serialize = "ATTj591169", to_string = "AttDsUiShellMaximum")]
    AttDsUiShellMaximum = 0x90541,
    #[strum(serialize = "ATTk131146", to_string = "AttDsaSignature")]
    AttDsaSignature = 0x2004a,
    #[strum(serialize = "ATTb590361", to_string = "AttDynamicLdapServer")]
    AttDynamicLdapServer = 0x90219,
    #[strum(serialize = "ATTm1376259", to_string = "AttEMailAddresses")]
    AttEMailAddresses = 0x150003,
    #[strum(serialize = "ATTk590092", to_string = "AttEfspolicy")]
    AttEfspolicy = 0x9010c,
    #[strum(serialize = "ATTm589859", to_string = "AttEmployeeId")]
    AttEmployeeId = 0x90023,
    #[strum(serialize = "ATTm131682", to_string = "AttEmployeeNumber")]
    AttEmployeeNumber = 0x20262,
    #[strum(serialize = "ATTm131685", to_string = "AttEmployeeType")]
    AttEmployeeType = 0x20265,
    #[strum(serialize = "ATTi131629", to_string = "AttEnabled")]
    AttEnabled = 0x2022d,
    #[strum(serialize = "ATTi589860", to_string = "AttEnabledConnection")]
    AttEnabledConnection = 0x90024,
    #[strum(serialize = "ATTm590649", to_string = "AttEnrollmentProviders")]
    AttEnrollmentProviders = 0x90339,
    #[strum(serialize = "ATTm590733", to_string = "AttExtendedAttributeInfo")]
    AttExtendedAttributeInfo = 0x9038d,
    #[strum(serialize = "ATTi131452", to_string = "AttExtendedCharsAllowed")]
    AttExtendedCharsAllowed = 0x2017c,
    #[strum(serialize = "ATTm590732", to_string = "AttExtendedClassInfo")]
    AttExtendedClassInfo = 0x9038c,
    #[strum(serialize = "ATTm131299", to_string = "AttExtensionName")]
    AttExtensionName = 0x200e3,
    #[strum(serialize = "ATTm591511", to_string = "AttExtraColumns")]
    AttExtraColumns = 0x90697,
    #[strum(serialize = "ATTm23", to_string = "AttFacsimileTelephoneNumber")]
    AttFacsimileTelephoneNumber = 0x17,
    #[strum(serialize = "ATTm590640", to_string = "AttFileExtPriority")]
    AttFileExtPriority = 0x90330,
    #[strum(serialize = "ATTj589862", to_string = "AttFlags")]
    AttFlags = 0x90026,
    #[strum(serialize = "ATTm590335", to_string = "AttFlatName")]
    AttFlatName = 0x901ff,
    #[strum(serialize = "ATTq589863", to_string = "AttForceLogoff")]
    AttForceLogoff = 0x90027,
    #[strum(serialize = "ATTk590180", to_string = "AttForeignIdentifier")]
    AttForeignIdentifier = 0x90164,
    #[strum(serialize = "ATTm590506", to_string = "AttFriendlyNames")]
    AttFriendlyNames = 0x902aa,
    #[strum(serialize = "ATTi590734", to_string = "AttFromEntry")]
    AttFromEntry = 0x9038e,
    #[strum(serialize = "ATTb589864", to_string = "AttFromServer")]
    AttFromServer = 0x90028,
    #[strum(serialize = "ATTb590693", to_string = "AttFrsComputerReference")]
    AttFrsComputerReference = 0x90365,
    #[strum(serialize = "ATTb590694", to_string = "AttFrsComputerReferenceBl")]
    AttFrsComputerReferenceBl = 0x90366,
    #[strum(serialize = "ATTm590695", to_string = "AttFrsControlDataCreation")]
    AttFrsControlDataCreation = 0x90367,
    #[strum(serialize = "ATTm590696", to_string = "AttFrsControlInboundBacklog")]
    AttFrsControlInboundBacklog = 0x90368,
    #[strum(serialize = "ATTm590697", to_string = "AttFrsControlOutboundBacklog")]
    AttFrsControlOutboundBacklog = 0x90369,
    #[strum(serialize = "ATTm590308", to_string = "AttFrsDirectoryFilter")]
    AttFrsDirectoryFilter = 0x901e4,
    #[strum(serialize = "ATTj590314", to_string = "AttFrsDsPoll")]
    AttFrsDsPoll = 0x901ea,
    #[strum(serialize = "ATTk590360", to_string = "AttFrsExtensions")]
    AttFrsExtensions = 0x90218,
    #[strum(serialize = "ATTm590315", to_string = "AttFrsFaultCondition")]
    AttFrsFaultCondition = 0x901eb,
    #[strum(serialize = "ATTm590307", to_string = "AttFrsFileFilter")]
    AttFrsFileFilter = 0x901e3,
    #[strum(serialize = "ATTj590698", to_string = "AttFrsFlags")]
    AttFrsFlags = 0x9036a,
    #[strum(serialize = "ATTj590358", to_string = "AttFrsLevelLimit")]
    AttFrsLevelLimit = 0x90216,
    #[strum(serialize = "ATTb590699", to_string = "AttFrsMemberReference")]
    AttFrsMemberReference = 0x9036b,
    #[strum(serialize = "ATTb590700", to_string = "AttFrsMemberReferenceBl")]
    AttFrsMemberReferenceBl = 0x9036c,
    #[strum(serialize = "ATTj590701", to_string = "AttFrsPartnerAuthLevel")]
    AttFrsPartnerAuthLevel = 0x9036d,
    #[strum(serialize = "ATTb590702", to_string = "AttFrsPrimaryMember")]
    AttFrsPrimaryMember = 0x9036e,
    #[strum(serialize = "ATTk590357", to_string = "AttFrsReplicaSetGuid")]
    AttFrsReplicaSetGuid = 0x90215,
    #[strum(serialize = "ATTj589855", to_string = "AttFrsReplicaSetType")]
    AttFrsReplicaSetType = 0x9001f,
    #[strum(serialize = "ATTm590311", to_string = "AttFrsRootPath")]
    AttFrsRootPath = 0x901e7,
    #[strum(serialize = "ATTp590359", to_string = "AttFrsRootSecurity")]
    AttFrsRootSecurity = 0x90217,
    #[strum(serialize = "ATTm590324", to_string = "AttFrsServiceCommand")]
    AttFrsServiceCommand = 0x901f4,
    #[strum(serialize = "ATTm590703", to_string = "AttFrsServiceCommandStatus")]
    AttFrsServiceCommandStatus = 0x9036f,
    #[strum(serialize = "ATTm590312", to_string = "AttFrsStagingPath")]
    AttFrsStagingPath = 0x901e8,
    #[strum(serialize = "ATTl590704", to_string = "AttFrsTimeLastCommand")]
    AttFrsTimeLastCommand = 0x90370,
    #[strum(serialize = "ATTl590705", to_string = "AttFrsTimeLastConfigChange")]
    AttFrsTimeLastConfigChange = 0x90371,
    #[strum(serialize = "ATTj590309", to_string = "AttFrsUpdateTimeout")]
    AttFrsUpdateTimeout = 0x901e5,
    #[strum(serialize = "ATTm590706", to_string = "AttFrsVersion")]
    AttFrsVersion = 0x90372,
    #[strum(serialize = "ATTk589867", to_string = "AttFrsVersionGuid")]
    AttFrsVersionGuid = 0x9002b,
    #[strum(serialize = "ATTm590310", to_string = "AttFrsWorkingPath")]
    AttFrsWorkingPath = 0x901e6,
    #[strum(serialize = "ATTb590193", to_string = "AttFsmoRoleOwner")]
    AttFsmoRoleOwner = 0x90171,
    #[strum(serialize = "ATTj131373", to_string = "AttGarbageCollPeriod")]
    AttGarbageCollPeriod = 0x2012d,
    #[strum(serialize = "ATTi589865", to_string = "AttGeneratedConnection")]
    AttGeneratedConnection = 0x90029,
    #[strum(serialize = "ATTm44", to_string = "AttGenerationQualifier")]
    AttGenerationQualifier = 0x2c,
    #[strum(serialize = "ATTm42", to_string = "AttGivenName")]
    AttGivenName = 0x2a,
    #[strum(serialize = "ATTb591069", to_string = "AttGlobalAddressList")]
    AttGlobalAddressList = 0x904dd,
    #[strum(serialize = "ATTc131094", to_string = "AttGovernsId")]
    AttGovernsId = 0x20016,
    #[strum(serialize = "ATTm590715", to_string = "AttGpLink")]
    AttGpLink = 0x9037b,
    #[strum(serialize = "ATTj590716", to_string = "AttGpOptions")]
    AttGpOptions = 0x9037c,
    #[strum(serialize = "ATTm590718", to_string = "AttGpcFileSysPath")]
    AttGpcFileSysPath = 0x9037e,
    #[strum(serialize = "ATTj590717", to_string = "AttGpcFunctionalityVersion")]
    AttGpcFunctionalityVersion = 0x9037d,
    #[strum(serialize = "ATTm591172", to_string = "AttGpcMachineExtensionNames")]
    AttGpcMachineExtensionNames = 0x90544,
    #[strum(serialize = "ATTm591173", to_string = "AttGpcUserExtensionNames")]
    AttGpcUserExtensionNames = 0x90545,
    #[strum(serialize = "ATTm591518", to_string = "AttGpcWqlFilter")]
    AttGpcWqlFilter = 0x9069e,
    #[strum(serialize = "ATTj589976", to_string = "AttGroupAttributes")]
    AttGroupAttributes = 0x90098,
    #[strum(serialize = "ATTk589990", to_string = "AttGroupMembershipSam")]
    AttGroupMembershipSam = 0x900a6,
    #[strum(serialize = "ATTm590169", to_string = "AttGroupPriority")]
    AttGroupPriority = 0x90159,
    #[strum(serialize = "ATTj590574", to_string = "AttGroupType")]
    AttGroupType = 0x902ee,
    #[strum(serialize = "ATTm590168", to_string = "AttGroupsToIgnore")]
    AttGroupsToIgnore = 0x90158,
    #[strum(serialize = "ATTb131086", to_string = "AttHasMasterNcs")]
    AttHasMasterNcs = 0x2000e,
    #[strum(serialize = "ATTb131087", to_string = "AttHasPartialReplicaNcs")]
    AttHasPartialReplicaNcs = 0x2000f,
    #[strum(serialize = "ATTk131474", to_string = "AttHelpData16")]
    AttHelpData16 = 0x20192,
    #[strum(serialize = "ATTk131081", to_string = "AttHelpData32")]
    AttHelpData32 = 0x20009,
    #[strum(serialize = "ATTm131399", to_string = "AttHelpFileName")]
    AttHelpFileName = 0x20147,
    #[strum(serialize = "ATTi591604", to_string = "AttHideFromAb")]
    AttHideFromAb = 0x906f4,
    #[strum(serialize = "ATTm589868", to_string = "AttHomeDirectory")]
    AttHomeDirectory = 0x9002c,
    #[strum(serialize = "ATTm589869", to_string = "AttHomeDrive")]
    AttHomeDrive = 0x9002d,
    #[strum(serialize = "ATTm590043", to_string = "AttIconPath")]
    AttIconPath = 0x900db,
    #[strum(serialize = "ATTk590144", to_string = "AttImplementedCategories")]
    AttImplementedCategories = 0x90140,
    #[strum(serialize = "ATTm590505", to_string = "AttIndexedscopes")]
    AttIndexedscopes = 0x902a9,
    #[strum(serialize = "ATTm590363", to_string = "AttInitialAuthIncoming")]
    AttInitialAuthIncoming = 0x9021b,
    #[strum(serialize = "ATTm590364", to_string = "AttInitialAuthOutgoing")]
    AttInitialAuthOutgoing = 0x9021c,
    #[strum(serialize = "ATTm43", to_string = "AttInitials")]
    AttInitials = 0x2b,
    #[strum(serialize = "ATTj590671", to_string = "AttInstallUiLevel")]
    AttInstallUiLevel = 0x9034f,
    #[strum(serialize = "ATTj131073", to_string = "AttInstanceType")]
    AttInstanceType = 0x20001,
    #[strum(serialize = "ATTj591072", to_string = "AttInterSiteTopologyFailover")]
    AttInterSiteTopologyFailover = 0x904e0,
    #[strum(serialize = "ATTb591070", to_string = "AttInterSiteTopologyGenerator")]
    AttInterSiteTopologyGenerator = 0x904de,
    #[strum(serialize = "ATTj591071", to_string = "AttInterSiteTopologyRenew")]
    AttInterSiteTopologyRenew = 0x904df,
    #[strum(serialize = "ATTg25", to_string = "AttInternationalIsdnNumber")]
    AttInternationalIsdnNumber = 0x19,
    #[strum(serialize = "ATTk131187", to_string = "AttInvocationId")]
    AttInvocationId = 0x20073,
    #[strum(serialize = "ATTk590447", to_string = "AttIpsecData")]
    AttIpsecData = 0x9026f,
    #[strum(serialize = "ATTj590446", to_string = "AttIpsecDataType")]
    AttIpsecDataType = 0x9026e,
    #[strum(serialize = "ATTb590453", to_string = "AttIpsecFilterReference")]
    AttIpsecFilterReference = 0x90275,
    #[strum(serialize = "ATTm590445", to_string = "AttIpsecId")]
    AttIpsecId = 0x9026d,
    #[strum(serialize = "ATTb590450", to_string = "AttIpsecIsakmpReference")]
    AttIpsecIsakmpReference = 0x90272,
    #[strum(serialize = "ATTm590444", to_string = "AttIpsecName")]
    AttIpsecName = 0x9026c,
    #[strum(serialize = "ATTm590712", to_string = "AttIpsecNegotiationPolicyAction")]
    AttIpsecNegotiationPolicyAction = 0x90378,
    #[strum(serialize = "ATTb590452", to_string = "AttIpsecNegotiationPolicyReference")]
    AttIpsecNegotiationPolicyReference = 0x90274,
    #[strum(serialize = "ATTm590711", to_string = "AttIpsecNegotiationPolicyType")]
    AttIpsecNegotiationPolicyType = 0x90377,
    #[strum(serialize = "ATTb590451", to_string = "AttIpsecNfaReference")]
    AttIpsecNfaReference = 0x90273,
    #[strum(serialize = "ATTb590448", to_string = "AttIpsecOwnersReference")]
    AttIpsecOwnersReference = 0x90270,
    #[strum(serialize = "ATTb590341", to_string = "AttIpsecPolicyReference")]
    AttIpsecPolicyReference = 0x90205,
    #[strum(serialize = "ATTi590692", to_string = "AttIsCriticalSystemObject")]
    AttIsCriticalSystemObject = 0x90364,
    #[strum(serialize = "ATTi590485", to_string = "AttIsDefunct")]
    AttIsDefunct = 0x90295,
    #[strum(serialize = "ATTi131120", to_string = "AttIsDeleted")]
    AttIsDeleted = 0x20030,
    #[strum(serialize = "ATTi591036", to_string = "AttIsEphemeral")]
    AttIsEphemeral = 0x904bc,
    #[strum(serialize = "ATTb131174", to_string = "AttIsMemberOfDl")]
    AttIsMemberOfDl = 0x20066,
    #[strum(serialize = "ATTi590463", to_string = "AttIsMemberOfPartialAttributeSet")]
    AttIsMemberOfPartialAttributeSet = 0x9027f,
    #[strum(serialize = "ATTb590462", to_string = "AttIsPrivilegeHolder")]
    AttIsPrivilegeHolder = 0x9027e,
    #[strum(serialize = "ATTi131105", to_string = "AttIsSingleValued")]
    AttIsSingleValued = 0x20021,
    #[strum(serialize = "ATTk1376316", to_string = "AttJpegphoto")]
    AttJpegphoto = 0x15003c,
    #[strum(serialize = "ATTm589872", to_string = "AttKeywords")]
    AttKeywords = 0x90030,
    #[strum(serialize = "ATTe2", to_string = "AttKnowledgeInformation")]
    AttKnowledgeInformation = 0x2,
    #[strum(serialize = "ATTq590343", to_string = "AttLastBackupRestorationTime")]
    AttLastBackupRestorationTime = 0x90207,
    #[strum(serialize = "ATTq589874", to_string = "AttLastContentIndexed")]
    AttLastContentIndexed = 0x90032,
    #[strum(serialize = "ATTb590605", to_string = "AttLastKnownParent")]
    AttLastKnownParent = 0x9030d,
    #[strum(serialize = "ATTq589875", to_string = "AttLastLogoff")]
    AttLastLogoff = 0x90033,
    #[strum(serialize = "ATTq589876", to_string = "AttLastLogon")]
    AttLastLogon = 0x90034,
    #[strum(serialize = "ATTq591520", to_string = "AttLastLogonTimestamp")]
    AttLastLogonTimestamp = 0x906a0,
    #[strum(serialize = "ATTq589877", to_string = "AttLastSetTime")]
    AttLastSetTime = 0x90035,
    #[strum(serialize = "ATTm590154", to_string = "AttLastUpdateSequence")]
    AttLastUpdateSequence = 0x9014a,
    #[strum(serialize = "ATTm590667", to_string = "AttLdapAdminLimits")]
    AttLdapAdminLimits = 0x9034b,
    #[strum(serialize = "ATTm131532", to_string = "AttLdapDisplayName")]
    AttLdapDisplayName = 0x201cc,
    #[strum(serialize = "ATTk590668", to_string = "AttLdapIpdenyList")]
    AttLdapIpdenyList = 0x9034c,
    #[strum(serialize = "ATTe590479", to_string = "AttLegacyExchangeDn")]
    AttLegacyExchangeDn = 0x9028f,
    #[strum(serialize = "ATTj131122", to_string = "AttLinkId")]
    AttLinkId = 0x20032,
    #[strum(serialize = "ATTk590093", to_string = "AttLinkTrackSecret")]
    AttLinkTrackSecret = 0x9010d,
    #[strum(serialize = "ATTk589984", to_string = "AttLmPwdHistory")]
    AttLmPwdHistory = 0x900a0,
    #[strum(serialize = "ATTj589880", to_string = "AttLocalPolicyFlags")]
    AttLocalPolicyFlags = 0x90038,
    #[strum(serialize = "ATTb590281", to_string = "AttLocalPolicyReference")]
    AttLocalPolicyReference = 0x901c9,
    #[strum(serialize = "ATTj589882", to_string = "AttLocaleId")]
    AttLocaleId = 0x9003a,
    #[strum(serialize = "ATTm7", to_string = "AttLocalityName")]
    AttLocalityName = 0x7,
    #[strum(serialize = "ATTm590641", to_string = "AttLocalizedDescription")]
    AttLocalizedDescription = 0x90331,
    #[strum(serialize = "ATTj591177", to_string = "AttLocalizationDisplayId")]
    AttLocalizationDisplayId = 0x90549,
    #[strum(serialize = "ATTm590046", to_string = "AttLocation")]
    AttLocation = 0x900de,
    #[strum(serialize = "ATTq589885", to_string = "AttLockOutObservationWindow")]
    AttLockOutObservationWindow = 0x9003d,
    #[strum(serialize = "ATTq589884", to_string = "AttLockoutDuration")]
    AttLockoutDuration = 0x9003c,
    #[strum(serialize = "ATTj589897", to_string = "AttLockoutThreshold")]
    AttLockoutThreshold = 0x90049,
    #[strum(serialize = "ATTq590486", to_string = "AttLockoutTime")]
    AttLockoutTime = 0x90296,
    #[strum(serialize = "ATTk1441828", to_string = "AttLogo")]
    AttLogo = 0x160024,
    #[strum(serialize = "ATTj589993", to_string = "AttLogonCount")]
    AttLogonCount = 0x900a9,
    #[strum(serialize = "ATTk589888", to_string = "AttLogonHours")]
    AttLogonHours = 0x90040,
    #[strum(serialize = "ATTk589889", to_string = "AttLogonWorkstation")]
    AttLogonWorkstation = 0x90041,
    #[strum(serialize = "ATTq589890", to_string = "AttLsaCreationTime")]
    AttLsaCreationTime = 0x90042,
    #[strum(serialize = "ATTq589891", to_string = "AttLsaModifiedCount")]
    AttLsaModifiedCount = 0x90043,
    #[strum(serialize = "ATTj589892", to_string = "AttMachineArchitecture")]
    AttMachineArchitecture = 0x90044,
    #[strum(serialize = "ATTq590344", to_string = "AttMachinePasswordChangeInterval")]
    AttMachinePasswordChangeInterval = 0x90208,
    #[strum(serialize = "ATTj589895", to_string = "AttMachineRole")]
    AttMachineRole = 0x90047,
    #[strum(serialize = "ATTk590283", to_string = "AttMachineWidePolicy")]
    AttMachineWidePolicy = 0x901cb,
    #[strum(serialize = "ATTb590477", to_string = "AttManagedBy")]
    AttManagedBy = 0x9028d,
    #[strum(serialize = "ATTb590478", to_string = "AttManagedObjects")]
    AttManagedObjects = 0x9028e,
    #[strum(serialize = "ATTb1376266", to_string = "AttManager")]
    AttManager = 0x15000a,
    #[strum(serialize = "ATTj131121", to_string = "AttMapiId")]
    AttMapiId = 0x20031,
    #[strum(serialize = "ATTk589896", to_string = "AttMarshalledInterface")]
    AttMarshalledInterface = 0x90048,
    #[strum(serialize = "ATTb591233", to_string = "AttMasteredBy")]
    AttMasteredBy = 0x90581,
    #[strum(serialize = "ATTq589898", to_string = "AttMaxPwdAge")]
    AttMaxPwdAge = 0x9004a,
    #[strum(serialize = "ATTq589899", to_string = "AttMaxRenewAge")]
    AttMaxRenewAge = 0x9004b,
    #[strum(serialize = "ATTq589900", to_string = "AttMaxStorage")]
    AttMaxStorage = 0x9004c,
    #[strum(serialize = "ATTq589901", to_string = "AttMaxTicketAge")]
    AttMaxTicketAge = 0x9004d,
    #[strum(serialize = "ATTc131097", to_string = "AttMayContain")]
    AttMayContain = 0x20019,
    #[strum(serialize = "ATTm590406", to_string = "AttMeetingadvertisescope")]
    AttMeetingadvertisescope = 0x90246,
    #[strum(serialize = "ATTm590397", to_string = "AttMeetingapplication")]
    AttMeetingapplication = 0x9023d,
    #[strum(serialize = "ATTj590413", to_string = "AttMeetingbandwidth")]
    AttMeetingbandwidth = 0x9024d,
    #[strum(serialize = "ATTk590414", to_string = "AttMeetingblob")]
    AttMeetingblob = 0x9024e,
    #[strum(serialize = "ATTm590402", to_string = "AttMeetingcontactinfo")]
    AttMeetingcontactinfo = 0x90242,
    #[strum(serialize = "ATTm590391", to_string = "AttMeetingdescription")]
    AttMeetingdescription = 0x90237,
    #[strum(serialize = "ATTl590412", to_string = "AttMeetingendtime")]
    AttMeetingendtime = 0x9024c,
    #[strum(serialize = "ATTm590389", to_string = "AttMeetingid")]
    AttMeetingid = 0x90235,
    #[strum(serialize = "ATTm590404", to_string = "AttMeetingip")]
    AttMeetingip = 0x90244,
    #[strum(serialize = "ATTm590409", to_string = "AttMeetingisencrypted")]
    AttMeetingisencrypted = 0x90249,
    #[strum(serialize = "ATTm590392", to_string = "AttMeetingkeyword")]
    AttMeetingkeyword = 0x90238,
    #[strum(serialize = "ATTm590398", to_string = "AttMeetinglanguage")]
    AttMeetinglanguage = 0x9023e,
    #[strum(serialize = "ATTm590393", to_string = "AttMeetinglocation")]
    AttMeetinglocation = 0x90239,
    #[strum(serialize = "ATTj590400", to_string = "AttMeetingmaxparticipants")]
    AttMeetingmaxparticipants = 0x90240,
    #[strum(serialize = "ATTm590390", to_string = "AttMeetingname")]
    AttMeetingname = 0x90236,
    #[strum(serialize = "ATTm590401", to_string = "AttMeetingoriginator")]
    AttMeetingoriginator = 0x90241,
    #[strum(serialize = "ATTm590403", to_string = "AttMeetingowner")]
    AttMeetingowner = 0x90243,
    #[strum(serialize = "ATTm590394", to_string = "AttMeetingprotocol")]
    AttMeetingprotocol = 0x9023a,
    #[strum(serialize = "ATTm590408", to_string = "AttMeetingrating")]
    AttMeetingrating = 0x90248,
    #[strum(serialize = "ATTm590410", to_string = "AttMeetingrecurrence")]
    AttMeetingrecurrence = 0x9024a,
    #[strum(serialize = "ATTm590405", to_string = "AttMeetingscope")]
    AttMeetingscope = 0x90245,
    #[strum(serialize = "ATTl590411", to_string = "AttMeetingstarttime")]
    AttMeetingstarttime = 0x9024b,
    #[strum(serialize = "ATTm590395", to_string = "AttMeetingtype")]
    AttMeetingtype = 0x9023b,
    #[strum(serialize = "ATTm590407", to_string = "AttMeetingurl")]
    AttMeetingurl = 0x90247,
    #[strum(serialize = "ATTb31", to_string = "AttMember")]
    AttMember = 0x1f,
    #[strum(serialize = "ATTm590474", to_string = "AttMhsOrAddress")]
    AttMhsOrAddress = 0x9028a,
    #[strum(serialize = "ATTq589902", to_string = "AttMinPwdAge")]
    AttMinPwdAge = 0x9004e,
    #[strum(serialize = "ATTj589903", to_string = "AttMinPwdLength")]
    AttMinPwdLength = 0x9004f,
    #[strum(serialize = "ATTq589904", to_string = "AttMinTicketAge")]
    AttMinTicketAge = 0x90050,
    #[strum(serialize = "ATTq589992", to_string = "AttModifiedCount")]
    AttModifiedCount = 0x900a8,
    #[strum(serialize = "ATTq589905", to_string = "AttModifiedCountAtLastProm")]
    AttModifiedCountAtLastProm = 0x90051,
    #[strum(serialize = "ATTl1638402", to_string = "AttModifyTimeStamp")]
    AttModifyTimeStamp = 0x190002,
    #[strum(serialize = "ATTk589906", to_string = "AttMoniker")]
    AttMoniker = 0x90052,
    #[strum(serialize = "ATTm589907", to_string = "AttMonikerDisplayName")]
    AttMonikerDisplayName = 0x90053,
    #[strum(serialize = "ATTk591129", to_string = "AttMoveTreeState")]
    AttMoveTreeState = 0x90519,
    #[strum(serialize = "ATTb591251", to_string = "AttMsComDefaultpartitionlink")]
    AttMsComDefaultpartitionlink = 0x90593,
    #[strum(serialize = "ATTk591252", to_string = "AttMsComObjectid")]
    AttMsComObjectid = 0x90594,
    #[strum(serialize = "ATTb591247", to_string = "AttMsComPartitionlink")]
    AttMsComPartitionlink = 0x9058f,
    #[strum(serialize = "ATTb591248", to_string = "AttMsComPartitionsetlink")]
    AttMsComPartitionsetlink = 0x90590,
    #[strum(serialize = "ATTb591249", to_string = "AttMsComUserlink")]
    AttMsComUserlink = 0x90591,
    #[strum(serialize = "ATTb591250", to_string = "AttMsComUserpartitionsetlink")]
    AttMsComUserpartitionsetlink = 0x90592,
    #[strum(serialize = "ATTm591541", to_string = "AttMsDsAdditionalDnsHostName")]
    AttMsDsAdditionalDnsHostName = 0x906b5,
    #[strum(serialize = "ATTm591542", to_string = "AttMsDsAdditionalSamAccountName")]
    AttMsDsAdditionalSamAccountName = 0x906b6,
    #[strum(serialize = "ATTj591613", to_string = "AttMsDsAllUsersTrustQuota")]
    AttMsDsAllUsersTrustQuota = 0x906fd,
    #[strum(serialize = "ATTm591534", to_string = "AttMsDsAllowedDnsSuffixes")]
    AttMsDsAllowedDnsSuffixes = 0x906ae,
    #[strum(serialize = "ATTm591611", to_string = "AttMsDsAllowedToDelegateTo")]
    AttMsDsAllowedToDelegateTo = 0x906fb,
    #[strum(serialize = "ATTc591282", to_string = "AttMsDsAuxiliaryClasses")]
    AttMsDsAuxiliaryClasses = 0x905b2,
    #[strum(serialize = "ATTj591493", to_string = "AttMsDsApproxImmedSubordinates")]
    AttMsDsApproxImmedSubordinates = 0x90685,
    #[strum(serialize = "ATTj591283", to_string = "AttMsDsBehaviorVersion")]
    AttMsDsBehaviorVersion = 0x905b3,
    #[strum(serialize = "ATTk591265", to_string = "AttMsDsCachedMembership")]
    AttMsDsCachedMembership = 0x905a1,
    #[strum(serialize = "ATTq591266", to_string = "AttMsDsCachedMembershipTimeStamp")]
    AttMsDsCachedMembershipTimeStamp = 0x905a2,
    #[strum(serialize = "ATTk591184", to_string = "AttMsDsConsistencyGuid")]
    AttMsDsConsistencyGuid = 0x90550,
    #[strum(serialize = "ATTj591185", to_string = "AttMsDsConsistencyChildCount")]
    AttMsDsConsistencyChildCount = 0x90551,
    #[strum(serialize = "ATTr591234", to_string = "AttMsDsCreatorSid")]
    AttMsDsCreatorSid = 0x90582,
    #[strum(serialize = "ATTm591543", to_string = "AttMsDsDnsrootalias")]
    AttMsDsDnsrootalias = 0x906b7,
    #[strum(serialize = "ATTl591446", to_string = "AttMsDsEntryTimeToDie")]
    AttMsDsEntryTimeToDie = 0x90656,
    #[strum(serialize = "ATTk591607", to_string = "AttMsDsExecutescriptpassword")]
    AttMsDsExecutescriptpassword = 0x906f7,
    #[strum(serialize = "ATTm591527", to_string = "AttMsDsFilterContainers")]
    AttMsDsFilterContainers = 0x906a7,
    #[strum(serialize = "ATTh591533", to_string = "AttMsDsHasInstantiatedNcs")]
    AttMsDsHasInstantiatedNcs = 0x906ad,
    #[strum(serialize = "ATTj591540", to_string = "AttMsDsIntid")]
    AttMsDsIntid = 0x906b4,
    #[strum(serialize = "ATTj591608", to_string = "AttMsDsLogonTimeSyncInterval")]
    AttMsDsLogonTimeSyncInterval = 0x906f8,
    #[strum(serialize = "ATTk591526", to_string = "AttMsDsTrustForestTrustInfo")]
    AttMsDsTrustForestTrustInfo = 0x906a6,
    #[strum(serialize = "ATTj591235", to_string = "AttMsDsMachineAccountQuota")]
    AttMsDsMachineAccountQuota = 0x90583,
    #[strum(serialize = "ATTm591445", to_string = "AttMsDsOtherSettings")]
    AttMsDsOtherSettings = 0x90655,
    #[strum(serialize = "ATTm591528", to_string = "AttMsDsNcReplCursors")]
    AttMsDsNcReplCursors = 0x906a8,
    #[strum(serialize = "ATTm591529", to_string = "AttMsDsNcReplInboundNeighbors")]
    AttMsDsNcReplInboundNeighbors = 0x906a9,
    #[strum(serialize = "ATTm591530", to_string = "AttMsDsNcReplOutboundNeighbors")]
    AttMsDsNcReplOutboundNeighbors = 0x906aa,
    #[strum(serialize = "ATTb591485", to_string = "AttMsDsNcReplicaLocations")]
    AttMsDsNcReplicaLocations = 0x9067d,
    #[strum(serialize = "ATTm591513", to_string = "AttMsDsNonSecurityGroupExtraClasses")]
    AttMsDsNonSecurityGroupExtraClasses = 0x90699,
    #[strum(serialize = "ATTj591612", to_string = "AttMsDsPerUserTrustQuota")]
    AttMsDsPerUserTrustQuota = 0x906fc,
    #[strum(serialize = "ATTj591614", to_string = "AttMsDsPerUserTrustTombstonesQuota")]
    AttMsDsPerUserTrustTombstonesQuota = 0x906fe,
    #[strum(serialize = "ATTb591268", to_string = "AttMsDsPreferredGcSite")]
    AttMsDsPreferredGcSite = 0x905a4,
    #[strum(serialize = "ATTm591531", to_string = "AttMsDsReplAttributeMetaData")]
    AttMsDsReplAttributeMetaData = 0x906ab,
    #[strum(serialize = "ATTm591532", to_string = "AttMsDsReplValueMetaData")]
    AttMsDsReplValueMetaData = 0x906ac,
    #[strum(serialize = "ATTh591232", to_string = "AttMsDsReplicatesNcReason")]
    AttMsDsReplicatesNcReason = 0x90580,
    #[strum(serialize = "ATTj591487", to_string = "AttMsDsReplicationNotifyFirstDsaDelay")]
    AttMsDsReplicationNotifyFirstDsaDelay = 0x9067f,
    #[strum(serialize = "ATTj591488", to_string = "AttMsDsReplicationNotifySubsequentDsaDelay")]
    AttMsDsReplicationNotifySubsequentDsaDelay = 0x90680,
    #[strum(serialize = "ATTj591544", to_string = "AttMsDsReplicationepoch")]
    AttMsDsReplicationepoch = 0x906b8,
    #[strum(serialize = "ATTk591264", to_string = "AttMsDsSchemaExtensions")]
    AttMsDsSchemaExtensions = 0x905a0,
    #[strum(serialize = "ATTb591535", to_string = "AttMsDsSdReferenceDomain")]
    AttMsDsSdReferenceDomain = 0x906af,
    #[strum(serialize = "ATTm591512", to_string = "AttMsDsSecurityGroupExtraClasses")]
    AttMsDsSecurityGroupExtraClasses = 0x90698,
    #[strum(serialize = "ATTm591521", to_string = "AttMsDsSettings")]
    AttMsDsSettings = 0x906a1,
    #[strum(serialize = "ATTk591267", to_string = "AttMsDsSiteAffinity")]
    AttMsDsSiteAffinity = 0x905a3,
    #[strum(serialize = "ATTm591539", to_string = "AttMsDsSpnSuffixes")]
    AttMsDsSpnSuffixes = 0x906b3,
    #[strum(serialize = "ATTj591284", to_string = "AttMsDsUserAccountControlComputed")]
    AttMsDsUserAccountControlComputed = 0x905b4,
    #[strum(serialize = "ATTm591545", to_string = "AttMsDsUpdatescript")]
    AttMsDsUpdatescript = 0x906b9,
    #[strum(serialize = "ATTm131516", to_string = "AttMsExchAssistantName")]
    AttMsExchAssistantName = 0x201bc,
    #[strum(serialize = "ATTm131665", to_string = "AttMsExchLabeleduri")]
    AttMsExchLabeleduri = 0x20251,
    #[strum(serialize = "ATTb131176", to_string = "AttMsExchOwnerBl")]
    AttMsExchOwnerBl = 0x20068,
    #[strum(serialize = "ATTb591517", to_string = "AttMsFrsHubMember")]
    AttMsFrsHubMember = 0x9069d,
    #[strum(serialize = "ATTm591516", to_string = "AttMsFrsTopologyPref")]
    AttMsFrsTopologyPref = 0x9069c,
    #[strum(serialize = "ATTm591610", to_string = "AttMsIisFtpDir")]
    AttMsIisFtpDir = 0x906fa,
    #[strum(serialize = "ATTm591609", to_string = "AttMsIisFtpRoot")]
    AttMsIisFtpRoot = 0x906f9,
    #[strum(serialize = "ATTk591548", to_string = "AttMsMmsData")]
    AttMsMmsData = 0x906bc,
    #[strum(serialize = "ATTk591549", to_string = "AttMsMmsIndex")]
    AttMsMmsIndex = 0x906bd,
    #[strum(serialize = "ATTk591550", to_string = "AttMsMmsIndice")]
    AttMsMmsIndice = 0x906be,
    #[strum(serialize = "ATTm591551", to_string = "AttMsMmsXml")]
    AttMsMmsXml = 0x906bf,
    #[strum(serialize = "ATTb591552", to_string = "AttMsMmsJoinLink")]
    AttMsMmsJoinLink = 0x906c0,
    #[strum(serialize = "ATTk591553", to_string = "AttMsMmsLineage")]
    AttMsMmsLineage = 0x906c1,
    #[strum(serialize = "ATTm591554", to_string = "AttMsMmsProvStatus")]
    AttMsMmsProvStatus = 0x906c2,
    #[strum(serialize = "ATTk591555", to_string = "AttMsMmsSyncStatus")]
    AttMsMmsSyncStatus = 0x906c3,
    #[strum(serialize = "ATTm591556", to_string = "AttMsMmsPartition")]
    AttMsMmsPartition = 0x906c4,
    #[strum(serialize = "ATTb591557", to_string = "AttMsMmsMaStagingLink")]
    AttMsMmsMaStagingLink = 0x906c5,
    #[strum(serialize = "ATTb591558", to_string = "AttMsMmsMaStagingBl")]
    AttMsMmsMaStagingBl = 0x906c6,
    #[strum(serialize = "ATTb591559", to_string = "AttMsMmsProvisioningLink")]
    AttMsMmsProvisioningLink = 0x906c7,
    #[strum(serialize = "ATTb591560", to_string = "AttMsMmsProvisioningBl")]
    AttMsMmsProvisioningBl = 0x906c8,
    #[strum(serialize = "ATTb591561", to_string = "AttMsMmsAssociatedLink")]
    AttMsMmsAssociatedLink = 0x906c9,
    #[strum(serialize = "ATTb591562", to_string = "AttMsMmsAssociatedBl")]
    AttMsMmsAssociatedBl = 0x906ca,
    #[strum(serialize = "ATTb591563", to_string = "AttMsMmsScopeLink")]
    AttMsMmsScopeLink = 0x906cb,
    #[strum(serialize = "ATTb591564", to_string = "AttMsMmsScopeBl")]
    AttMsMmsScopeBl = 0x906cc,
    #[strum(serialize = "ATTm591565", to_string = "AttMsMmsCriteria")]
    AttMsMmsCriteria = 0x906cd,
    #[strum(serialize = "ATTb591566", to_string = "AttMsMmsDomainController")]
    AttMsMmsDomainController = 0x906ce,
    #[strum(serialize = "ATTm591567", to_string = "AttMsMmsServiceName")]
    AttMsMmsServiceName = 0x906cf,
    #[strum(serialize = "ATTm591568", to_string = "AttMsMmsInstanceConfiguration")]
    AttMsMmsInstanceConfiguration = 0x906d0,
    #[strum(serialize = "ATTm591569", to_string = "AttMsMmsToolsConfiguration")]
    AttMsMmsToolsConfiguration = 0x906d1,
    #[strum(serialize = "ATTm591570", to_string = "AttMsMmsInstanceRule")]
    AttMsMmsInstanceRule = 0x906d2,
    #[strum(serialize = "ATTm591571", to_string = "AttMsMmsInstallStatus")]
    AttMsMmsInstallStatus = 0x906d3,
    #[strum(serialize = "ATTm591572", to_string = "AttMsMmsVersion")]
    AttMsMmsVersion = 0x906d4,
    #[strum(serialize = "ATTb591573", to_string = "AttMsMmsConnectorSpace")]
    AttMsMmsConnectorSpace = 0x906d5,
    #[strum(serialize = "ATTb591574", to_string = "AttMsMmsScope")]
    AttMsMmsScope = 0x906d6,
    #[strum(serialize = "ATTm591575", to_string = "AttMsMmsInstanceSchedule")]
    AttMsMmsInstanceSchedule = 0x906d7,
    #[strum(serialize = "ATTk591576", to_string = "AttMsMmsInstanceInfo")]
    AttMsMmsInstanceInfo = 0x906d8,
    #[strum(serialize = "ATTm591577", to_string = "AttMsMmsMaConfiguration")]
    AttMsMmsMaConfiguration = 0x906d9,
    #[strum(serialize = "ATTk591578", to_string = "AttMsMmsMaConfigurationPrivate")]
    AttMsMmsMaConfigurationPrivate = 0x906da,
    #[strum(serialize = "ATTm591579", to_string = "AttMsMmsMaSchema")]
    AttMsMmsMaSchema = 0x906db,
    #[strum(serialize = "ATTm591580", to_string = "AttMsMmsMaMap")]
    AttMsMmsMaMap = 0x906dc,
    #[strum(serialize = "ATTm591581", to_string = "AttMsMmsMaCapability")]
    AttMsMmsMaCapability = 0x906dd,
    #[strum(serialize = "ATTm591582", to_string = "AttMsMmsMaExecutionHistory")]
    AttMsMmsMaExecutionHistory = 0x906de,
    #[strum(serialize = "ATTm591583", to_string = "AttMsMmsMaCategory")]
    AttMsMmsMaCategory = 0x906df,
    #[strum(serialize = "ATTk591584", to_string = "AttMsMmsMaAdInfo")]
    AttMsMmsMaAdInfo = 0x906e0,
    #[strum(serialize = "ATTk591585", to_string = "AttMsMmsMaCdInfo")]
    AttMsMmsMaCdInfo = 0x906e1,
    #[strum(serialize = "ATTk591586", to_string = "AttMsMmsMaProcessInfo")]
    AttMsMmsMaProcessInfo = 0x906e2,
    #[strum(serialize = "ATTk591587", to_string = "AttMsMmsMaScriptInfo")]
    AttMsMmsMaScriptInfo = 0x906e3,
    #[strum(serialize = "ATTk591588", to_string = "AttMsMmsMaSystem")]
    AttMsMmsMaSystem = 0x906e4,
    #[strum(serialize = "ATTk591589", to_string = "AttMsMmsMaSynchronization")]
    AttMsMmsMaSynchronization = 0x906e5,
    #[strum(serialize = "ATTb591590", to_string = "AttMsMmsJoinBl")]
    AttMsMmsJoinBl = 0x906e6,
    #[strum(serialize = "ATTk591591", to_string = "AttMsMmsAnchor")]
    AttMsMmsAnchor = 0x906e7,
    #[strum(serialize = "ATTk591592", to_string = "AttMsMmsExportKey")]
    AttMsMmsExportKey = 0x906e8,
    #[strum(serialize = "ATTk591593", to_string = "AttMsMmsImportKey")]
    AttMsMmsImportKey = 0x906e9,
    #[strum(serialize = "ATTk591594", to_string = "AttMsMmsState")]
    AttMsMmsState = 0x906ea,
    #[strum(serialize = "ATTk591595", to_string = "AttMsMmsHologram")]
    AttMsMmsHologram = 0x906eb,
    #[strum(serialize = "ATTk591596", to_string = "AttMsMmsDeltaHologram")]
    AttMsMmsDeltaHologram = 0x906ec,
    #[strum(serialize = "ATTm591597", to_string = "AttMsMmsProvisioningConfiguration")]
    AttMsMmsProvisioningConfiguration = 0x906ed,
    #[strum(serialize = "ATTk591598", to_string = "AttMsMmsProvisioningConfigurationPrivate")]
    AttMsMmsProvisioningConfigurationPrivate = 0x906ee,
    #[strum(serialize = "ATTk591599", to_string = "AttMsMmsProvisioningAdInfo")]
    AttMsMmsProvisioningAdInfo = 0x906ef,
    #[strum(serialize = "ATTm591600", to_string = "AttMsMmsProvisioningSystem")]
    AttMsMmsProvisioningSystem = 0x906f0,
    #[strum(serialize = "ATTm591601", to_string = "AttMsMmsProvisioningStatusXml")]
    AttMsMmsProvisioningStatusXml = 0x906f1,
    #[strum(serialize = "ATTk591602", to_string = "AttMsMmsProvisioningStatusBinary")]
    AttMsMmsProvisioningStatusBinary = 0x906f2,
    #[strum(serialize = "ATTm591260", to_string = "AttMsPkiCertTemplateOid")]
    AttMsPkiCertTemplateOid = 0x9059c,
    #[strum(serialize = "ATTm591498", to_string = "AttMsPkiCertificateApplicationPolicy")]
    AttMsPkiCertificateApplicationPolicy = 0x9068a,
    #[strum(serialize = "ATTj591256", to_string = "AttMsPkiCertificateNameFlag")]
    AttMsPkiCertificateNameFlag = 0x90598,
    #[strum(serialize = "ATTm591263", to_string = "AttMsPkiCertificatePolicy")]
    AttMsPkiCertificatePolicy = 0x9059f,
    #[strum(serialize = "ATTj591254", to_string = "AttMsPkiEnrollmentFlag")]
    AttMsPkiEnrollmentFlag = 0x90596,
    #[strum(serialize = "ATTj591257", to_string = "AttMsPkiMinimalKeySize")]
    AttMsPkiMinimalKeySize = 0x90599,
    #[strum(serialize = "ATTj591495", to_string = "AttMsPkiOidAttribute")]
    AttMsPkiOidAttribute = 0x90687,
    #[strum(serialize = "ATTm591496", to_string = "AttMsPkiOidCps")]
    AttMsPkiOidCps = 0x90688,
    #[strum(serialize = "ATTm591536", to_string = "AttMsPkiOidLocalizedname")]
    AttMsPkiOidLocalizedname = 0x906b0,
    #[strum(serialize = "ATTm591497", to_string = "AttMsPkiOidUserNotice")]
    AttMsPkiOidUserNotice = 0x90689,
    #[strum(serialize = "ATTj591255", to_string = "AttMsPkiPrivateKeyFlag")]
    AttMsPkiPrivateKeyFlag = 0x90597,
    #[strum(serialize = "ATTm591261", to_string = "AttMsPkiSupersedeTemplates")]
    AttMsPkiSupersedeTemplates = 0x9059d,
    #[strum(serialize = "ATTj591259", to_string = "AttMsPkiTemplateMinorRevision")]
    AttMsPkiTemplateMinorRevision = 0x9059b,
    #[strum(serialize = "ATTj591258", to_string = "AttMsPkiTemplateSchemaVersion")]
    AttMsPkiTemplateSchemaVersion = 0x9059a,
    #[strum(serialize = "ATTm591499", to_string = "AttMsPkiRaApplicationPolicies")]
    AttMsPkiRaApplicationPolicies = 0x9068b,
    #[strum(serialize = "ATTm591262", to_string = "AttMsPkiRaPolicies")]
    AttMsPkiRaPolicies = 0x9059e,
    #[strum(serialize = "ATTj591253", to_string = "AttMsPkiRaSignature")]
    AttMsPkiRaSignature = 0x90595,
    #[strum(serialize = "ATTm590708", to_string = "AttMsRrasAttribute")]
    AttMsRrasAttribute = 0x90374,
    #[strum(serialize = "ATTm590707", to_string = "AttMsRrasVendorAttributeEntry")]
    AttMsRrasVendorAttributeEntry = 0x90373,
    #[strum(serialize = "ATTm591187", to_string = "AttMsSqlName")]
    AttMsSqlName = 0x90553,
    #[strum(serialize = "ATTm591188", to_string = "AttMsSqlRegisteredowner")]
    AttMsSqlRegisteredowner = 0x90554,
    #[strum(serialize = "ATTm591189", to_string = "AttMsSqlContact")]
    AttMsSqlContact = 0x90555,
    #[strum(serialize = "ATTm591190", to_string = "AttMsSqlLocation")]
    AttMsSqlLocation = 0x90556,
    #[strum(serialize = "ATTq591191", to_string = "AttMsSqlMemory")]
    AttMsSqlMemory = 0x90557,
    #[strum(serialize = "ATTj591192", to_string = "AttMsSqlBuild")]
    AttMsSqlBuild = 0x90558,
    #[strum(serialize = "ATTm591193", to_string = "AttMsSqlServiceaccount")]
    AttMsSqlServiceaccount = 0x90559,
    #[strum(serialize = "ATTj591194", to_string = "AttMsSqlCharacterset")]
    AttMsSqlCharacterset = 0x9055a,
    #[strum(serialize = "ATTm591195", to_string = "AttMsSqlSortorder")]
    AttMsSqlSortorder = 0x9055b,
    #[strum(serialize = "ATTj591196", to_string = "AttMsSqlUnicodesortorder")]
    AttMsSqlUnicodesortorder = 0x9055c,
    #[strum(serialize = "ATTi591197", to_string = "AttMsSqlClustered")]
    AttMsSqlClustered = 0x9055d,
    #[strum(serialize = "ATTm591198", to_string = "AttMsSqlNamedpipe")]
    AttMsSqlNamedpipe = 0x9055e,
    #[strum(serialize = "ATTm591199", to_string = "AttMsSqlMultiprotocol")]
    AttMsSqlMultiprotocol = 0x9055f,
    #[strum(serialize = "ATTm591200", to_string = "AttMsSqlSpx")]
    AttMsSqlSpx = 0x90560,
    #[strum(serialize = "ATTm591201", to_string = "AttMsSqlTcpip")]
    AttMsSqlTcpip = 0x90561,
    #[strum(serialize = "ATTm591202", to_string = "AttMsSqlAppletalk")]
    AttMsSqlAppletalk = 0x90562,
    #[strum(serialize = "ATTm591203", to_string = "AttMsSqlVines")]
    AttMsSqlVines = 0x90563,
    #[strum(serialize = "ATTq591204", to_string = "AttMsSqlStatus")]
    AttMsSqlStatus = 0x90564,
    #[strum(serialize = "ATTm591205", to_string = "AttMsSqlLastupdateddate")]
    AttMsSqlLastupdateddate = 0x90565,
    #[strum(serialize = "ATTm591206", to_string = "AttMsSqlInformationurl")]
    AttMsSqlInformationurl = 0x90566,
    #[strum(serialize = "ATTm591207", to_string = "AttMsSqlConnectionurl")]
    AttMsSqlConnectionurl = 0x90567,
    #[strum(serialize = "ATTm591208", to_string = "AttMsSqlPublicationurl")]
    AttMsSqlPublicationurl = 0x90568,
    #[strum(serialize = "ATTm591209", to_string = "AttMsSqlGpslatitude")]
    AttMsSqlGpslatitude = 0x90569,
    #[strum(serialize = "ATTm591210", to_string = "AttMsSqlGpslongitude")]
    AttMsSqlGpslongitude = 0x9056a,
    #[strum(serialize = "ATTm591211", to_string = "AttMsSqlGpsheight")]
    AttMsSqlGpsheight = 0x9056b,
    #[strum(serialize = "ATTm591212", to_string = "AttMsSqlVersion")]
    AttMsSqlVersion = 0x9056c,
    #[strum(serialize = "ATTm591213", to_string = "AttMsSqlLanguage")]
    AttMsSqlLanguage = 0x9056d,
    #[strum(serialize = "ATTm591214", to_string = "AttMsSqlDescription")]
    AttMsSqlDescription = 0x9056e,
    #[strum(serialize = "ATTm591215", to_string = "AttMsSqlType")]
    AttMsSqlType = 0x9056f,
    #[strum(serialize = "ATTi591216", to_string = "AttMsSqlInformationdirectory")]
    AttMsSqlInformationdirectory = 0x90570,
    #[strum(serialize = "ATTm591217", to_string = "AttMsSqlDatabase")]
    AttMsSqlDatabase = 0x90571,
    #[strum(serialize = "ATTi591218", to_string = "AttMsSqlAllowanonymoussubscription")]
    AttMsSqlAllowanonymoussubscription = 0x90572,
    #[strum(serialize = "ATTm591219", to_string = "AttMsSqlAlias")]
    AttMsSqlAlias = 0x90573,
    #[strum(serialize = "ATTq591220", to_string = "AttMsSqlSize")]
    AttMsSqlSize = 0x90574,
    #[strum(serialize = "ATTm591221", to_string = "AttMsSqlCreationdate")]
    AttMsSqlCreationdate = 0x90575,
    #[strum(serialize = "ATTm591222", to_string = "AttMsSqlLastbackupdate")]
    AttMsSqlLastbackupdate = 0x90576,
    #[strum(serialize = "ATTm591223", to_string = "AttMsSqlLastdiagnosticdate")]
    AttMsSqlLastdiagnosticdate = 0x90577,
    #[strum(serialize = "ATTm591224", to_string = "AttMsSqlApplications")]
    AttMsSqlApplications = 0x90578,
    #[strum(serialize = "ATTm591225", to_string = "AttMsSqlKeywords")]
    AttMsSqlKeywords = 0x90579,
    #[strum(serialize = "ATTm591226", to_string = "AttMsSqlPublisher")]
    AttMsSqlPublisher = 0x9057a,
    #[strum(serialize = "ATTi591227", to_string = "AttMsSqlAllowknownpullsubscription")]
    AttMsSqlAllowknownpullsubscription = 0x9057b,
    #[strum(serialize = "ATTi591228", to_string = "AttMsSqlAllowimmediateupdatingsubscription")]
    AttMsSqlAllowimmediateupdatingsubscription = 0x9057c,
    #[strum(serialize = "ATTi591229", to_string = "AttMsSqlAllowqueuedupdatingsubscription")]
    AttMsSqlAllowqueuedupdatingsubscription = 0x9057d,
    #[strum(serialize = "ATTi591230", to_string = "AttMsSqlAllowsnapshotfilesftpdownloading")]
    AttMsSqlAllowsnapshotfilesftpdownloading = 0x9057e,
    #[strum(serialize = "ATTi591231", to_string = "AttMsSqlThirdparty")]
    AttMsSqlThirdparty = 0x9057f,
    #[strum(serialize = "ATTk591524", to_string = "AttMsTapiConferenceBlob")]
    AttMsTapiConferenceBlob = 0x906a4,
    #[strum(serialize = "ATTm591525", to_string = "AttMsTapiIpAddress")]
    AttMsTapiIpAddress = 0x906a5,
    #[strum(serialize = "ATTm591523", to_string = "AttMsTapiProtocolId")]
    AttMsTapiProtocolId = 0x906a3,
    #[strum(serialize = "ATTm591522", to_string = "AttMsTapiUniqueIdentifier")]
    AttMsTapiUniqueIdentifier = 0x906a2,
    #[strum(serialize = "ATTm591447", to_string = "AttMsWmiAuthor")]
    AttMsWmiAuthor = 0x90657,
    #[strum(serialize = "ATTm591448", to_string = "AttMsWmiChangedate")]
    AttMsWmiChangedate = 0x90658,
    #[strum(serialize = "ATTm591500", to_string = "AttMsWmiClass")]
    AttMsWmiClass = 0x9068c,
    #[strum(serialize = "ATTm591449", to_string = "AttMsWmiClassdefinition")]
    AttMsWmiClassdefinition = 0x90659,
    #[strum(serialize = "ATTm591450", to_string = "AttMsWmiCreationdate")]
    AttMsWmiCreationdate = 0x9065a,
    #[strum(serialize = "ATTj591501", to_string = "AttMsWmiGenus")]
    AttMsWmiGenus = 0x9068d,
    #[strum(serialize = "ATTm591451", to_string = "AttMsWmiId")]
    AttMsWmiId = 0x9065b,
    #[strum(serialize = "ATTj591452", to_string = "AttMsWmiIntdefault")]
    AttMsWmiIntdefault = 0x9065c,
    #[strum(serialize = "ATTj591502", to_string = "AttMsWmiIntflags1")]
    AttMsWmiIntflags1 = 0x9068e,
    #[strum(serialize = "ATTj591503", to_string = "AttMsWmiIntflags2")]
    AttMsWmiIntflags2 = 0x9068f,
    #[strum(serialize = "ATTj591504", to_string = "AttMsWmiIntflags3")]
    AttMsWmiIntflags3 = 0x90690,
    #[strum(serialize = "ATTj591505", to_string = "AttMsWmiIntflags4")]
    AttMsWmiIntflags4 = 0x90691,
    #[strum(serialize = "ATTj591453", to_string = "AttMsWmiIntmax")]
    AttMsWmiIntmax = 0x9065d,
    #[strum(serialize = "ATTj591454", to_string = "AttMsWmiIntmin")]
    AttMsWmiIntmin = 0x9065e,
    #[strum(serialize = "ATTj591455", to_string = "AttMsWmiIntvalidvalues")]
    AttMsWmiIntvalidvalues = 0x9065f,
    #[strum(serialize = "ATTq591456", to_string = "AttMsWmiInt8Default")]
    AttMsWmiInt8Default = 0x90660,
    #[strum(serialize = "ATTq591457", to_string = "AttMsWmiInt8Max")]
    AttMsWmiInt8Max = 0x90661,
    #[strum(serialize = "ATTq591458", to_string = "AttMsWmiInt8Min")]
    AttMsWmiInt8Min = 0x90662,
    #[strum(serialize = "ATTq591459", to_string = "AttMsWmiInt8Validvalues")]
    AttMsWmiInt8Validvalues = 0x90663,
    #[strum(serialize = "ATTm591462", to_string = "AttMsWmiMof")]
    AttMsWmiMof = 0x90666,
    #[strum(serialize = "ATTm591463", to_string = "AttMsWmiName")]
    AttMsWmiName = 0x90667,
    #[strum(serialize = "ATTm591464", to_string = "AttMsWmiNormalizedclass")]
    AttMsWmiNormalizedclass = 0x90668,
    #[strum(serialize = "ATTm591506", to_string = "AttMsWmiParm1")]
    AttMsWmiParm1 = 0x90692,
    #[strum(serialize = "ATTm591507", to_string = "AttMsWmiParm2")]
    AttMsWmiParm2 = 0x90693,
    #[strum(serialize = "ATTm591508", to_string = "AttMsWmiParm3")]
    AttMsWmiParm3 = 0x90694,
    #[strum(serialize = "ATTm591509", to_string = "AttMsWmiParm4")]
    AttMsWmiParm4 = 0x90695,
    #[strum(serialize = "ATTm591465", to_string = "AttMsWmiPropertyname")]
    AttMsWmiPropertyname = 0x90669,
    #[strum(serialize = "ATTm591466", to_string = "AttMsWmiQuery")]
    AttMsWmiQuery = 0x9066a,
    #[strum(serialize = "ATTm591467", to_string = "AttMsWmiQuerylanguage")]
    AttMsWmiQuerylanguage = 0x9066b,
    #[strum(serialize = "ATTm591510", to_string = "AttMsWmiScopeguid")]
    AttMsWmiScopeguid = 0x90696,
    #[strum(serialize = "ATTm591468", to_string = "AttMsWmiSourceorganization")]
    AttMsWmiSourceorganization = 0x9066c,
    #[strum(serialize = "ATTm591460", to_string = "AttMsWmiStringdefault")]
    AttMsWmiStringdefault = 0x90664,
    #[strum(serialize = "ATTm591461", to_string = "AttMsWmiStringvalidvalues")]
    AttMsWmiStringvalidvalues = 0x90665,
    #[strum(serialize = "ATTm591469", to_string = "AttMsWmiTargetclass")]
    AttMsWmiTargetclass = 0x9066d,
    #[strum(serialize = "ATTm591470", to_string = "AttMsWmiTargetnamespace")]
    AttMsWmiTargetnamespace = 0x9066e,
    #[strum(serialize = "ATTk591471", to_string = "AttMsWmiTargetobject")]
    AttMsWmiTargetobject = 0x9066f,
    #[strum(serialize = "ATTm591472", to_string = "AttMsWmiTargetpath")]
    AttMsWmiTargetpath = 0x90670,
    #[strum(serialize = "ATTm591473", to_string = "AttMsWmiTargettype")]
    AttMsWmiTargettype = 0x90671,
    #[strum(serialize = "ATTf590540", to_string = "AttMscopeId")]
    AttMscopeId = 0x902cc,
    #[strum(serialize = "ATTm590495", to_string = "AttMsiFileList")]
    AttMsiFileList = 0x9029f,
    #[strum(serialize = "ATTk590638", to_string = "AttMsiScript")]
    AttMsiScript = 0x9032e,
    #[strum(serialize = "ATTm590669", to_string = "AttMsiScriptName")]
    AttMsiScriptName = 0x9034d,
    #[strum(serialize = "ATTm589839", to_string = "AttMsiScriptPath")]
    AttMsiScriptPath = 0x9000f,
    #[strum(serialize = "ATTj590670", to_string = "AttMsiScriptSize")]
    AttMsiScriptSize = 0x9034e,
    #[strum(serialize = "ATTi590747", to_string = "AttMsmqAuthenticate")]
    AttMsmqAuthenticate = 0x9039b,
    #[strum(serialize = "ATTj590744", to_string = "AttMsmqBasePriority")]
    AttMsmqBasePriority = 0x90398,
    #[strum(serialize = "ATTe590757", to_string = "AttMsmqComputerType")]
    AttMsmqComputerType = 0x903a5,
    #[strum(serialize = "ATTm591241", to_string = "AttMsmqComputerTypeEx")]
    AttMsmqComputerTypeEx = 0x90589,
    #[strum(serialize = "ATTj590770", to_string = "AttMsmqCost")]
    AttMsmqCost = 0x903b2,
    #[strum(serialize = "ATTe590764", to_string = "AttMsmqCspName")]
    AttMsmqCspName = 0x903ac,
    #[strum(serialize = "ATTi591063", to_string = "AttMsmqDependentClientService")]
    AttMsmqDependentClientService = 0x904d7,
    #[strum(serialize = "ATTi591050", to_string = "AttMsmqDependentClientServices")]
    AttMsmqDependentClientServices = 0x904ca,
    #[strum(serialize = "ATTk590772", to_string = "AttMsmqDigests")]
    AttMsmqDigests = 0x903b4,
    #[strum(serialize = "ATTk590790", to_string = "AttMsmqDigestsMig")]
    AttMsmqDigestsMig = 0x903c6,
    #[strum(serialize = "ATTi591062", to_string = "AttMsmqDsService")]
    AttMsmqDsService = 0x904d6,
    #[strum(serialize = "ATTi591052", to_string = "AttMsmqDsServices")]
    AttMsmqDsServices = 0x904cc,
    #[strum(serialize = "ATTk590760", to_string = "AttMsmqEncryptKey")]
    AttMsmqEncryptKey = 0x903a8,
    #[strum(serialize = "ATTi590758", to_string = "AttMsmqForeign")]
    AttMsmqForeign = 0x903a6,
    #[strum(serialize = "ATTb590753", to_string = "AttMsmqInRoutingServers")]
    AttMsmqInRoutingServers = 0x903a1,
    #[strum(serialize = "ATTj591132", to_string = "AttMsmqInterval1")]
    AttMsmqInterval1 = 0x9051c,
    #[strum(serialize = "ATTj591133", to_string = "AttMsmqInterval2")]
    AttMsmqInterval2 = 0x9051d,
    #[strum(serialize = "ATTi590742", to_string = "AttMsmqJournal")]
    AttMsmqJournal = 0x90396,
    #[strum(serialize = "ATTj590745", to_string = "AttMsmqJournalQuota")]
    AttMsmqJournalQuota = 0x90399,
    #[strum(serialize = "ATTe590746", to_string = "AttMsmqLabel")]
    AttMsmqLabel = 0x9039a,
    #[strum(serialize = "ATTm591239", to_string = "AttMsmqLabelEx")]
    AttMsmqLabelEx = 0x90587,
    #[strum(serialize = "ATTj590765", to_string = "AttMsmqLongLived")]
    AttMsmqLongLived = 0x903ad,
    #[strum(serialize = "ATTi590776", to_string = "AttMsmqMigrated")]
    AttMsmqMigrated = 0x903b8,
    #[strum(serialize = "ATTm591538", to_string = "AttMsmqMulticastAddress")]
    AttMsmqMulticastAddress = 0x906b2,
    #[strum(serialize = "ATTi590763", to_string = "AttMsmqNameStyle")]
    AttMsmqNameStyle = 0x903ab,
    #[strum(serialize = "ATTj590788", to_string = "AttMsmqNt4Flags")]
    AttMsmqNt4Flags = 0x903c4,
    #[strum(serialize = "ATTj590784", to_string = "AttMsmqNt4Stub")]
    AttMsmqNt4Stub = 0x903c0,
    #[strum(serialize = "ATTj590759", to_string = "AttMsmqOsType")]
    AttMsmqOsType = 0x903a7,
    #[strum(serialize = "ATTb590752", to_string = "AttMsmqOutRoutingServers")]
    AttMsmqOutRoutingServers = 0x903a0,
    #[strum(serialize = "ATTk590749", to_string = "AttMsmqOwnerId")]
    AttMsmqOwnerId = 0x9039d,
    #[strum(serialize = "ATTb591049", to_string = "AttMsmqPrevSiteGates")]
    AttMsmqPrevSiteGates = 0x904c9,
    #[strum(serialize = "ATTj590748", to_string = "AttMsmqPrivacyLevel")]
    AttMsmqPrivacyLevel = 0x9039c,
    #[strum(serialize = "ATTk590775", to_string = "AttMsmqQmId")]
    AttMsmqQmId = 0x903b7,
    #[strum(serialize = "ATTj590787", to_string = "AttMsmqQueueJournalQuota")]
    AttMsmqQueueJournalQuota = 0x903c3,
    #[strum(serialize = "ATTm591067", to_string = "AttMsmqQueueNameExt")]
    AttMsmqQueueNameExt = 0x904db,
    #[strum(serialize = "ATTj590786", to_string = "AttMsmqQueueQuota")]
    AttMsmqQueueQuota = 0x903c2,
    #[strum(serialize = "ATTk590741", to_string = "AttMsmqQueueType")]
    AttMsmqQueueType = 0x90395,
    #[strum(serialize = "ATTj590743", to_string = "AttMsmqQuota")]
    AttMsmqQuota = 0x90397,
    #[strum(serialize = "ATTm591519", to_string = "AttMsmqRecipientFormatname")]
    AttMsmqRecipientFormatname = 0x9069f,
    #[strum(serialize = "ATTi591061", to_string = "AttMsmqRoutingService")]
    AttMsmqRoutingService = 0x904d5,
    #[strum(serialize = "ATTi591051", to_string = "AttMsmqRoutingServices")]
    AttMsmqRoutingServices = 0x904cb,
    #[strum(serialize = "ATTi591537", to_string = "AttMsmqSecuredSource")]
    AttMsmqSecuredSource = 0x906b1,
    #[strum(serialize = "ATTj590754", to_string = "AttMsmqServiceType")]
    AttMsmqServiceType = 0x903a2,
    #[strum(serialize = "ATTj590774", to_string = "AttMsmqServices")]
    AttMsmqServices = 0x903b6,
    #[strum(serialize = "ATTk590771", to_string = "AttMsmqSignCertificates")]
    AttMsmqSignCertificates = 0x903b3,
    #[strum(serialize = "ATTk590791", to_string = "AttMsmqSignCertificatesMig")]
    AttMsmqSignCertificatesMig = 0x903c7,
    #[strum(serialize = "ATTk590761", to_string = "AttMsmqSignKey")]
    AttMsmqSignKey = 0x903a9,
    #[strum(serialize = "ATTb590767", to_string = "AttMsmqSite1")]
    AttMsmqSite1 = 0x903af,
    #[strum(serialize = "ATTb590768", to_string = "AttMsmqSite2")]
    AttMsmqSite2 = 0x903b0,
    #[strum(serialize = "ATTi590785", to_string = "AttMsmqSiteForeign")]
    AttMsmqSiteForeign = 0x903c1,
    #[strum(serialize = "ATTb590769", to_string = "AttMsmqSiteGates")]
    AttMsmqSiteGates = 0x903b1,
    #[strum(serialize = "ATTb591134", to_string = "AttMsmqSiteGatesMig")]
    AttMsmqSiteGatesMig = 0x9051e,
    #[strum(serialize = "ATTk590777", to_string = "AttMsmqSiteId")]
    AttMsmqSiteId = 0x903b9,
    #[strum(serialize = "ATTe590789", to_string = "AttMsmqSiteName")]
    AttMsmqSiteName = 0x903c5,
    #[strum(serialize = "ATTm591240", to_string = "AttMsmqSiteNameEx")]
    AttMsmqSiteNameEx = 0x90588,
    #[strum(serialize = "ATTk590751", to_string = "AttMsmqSites")]
    AttMsmqSites = 0x9039f,
    #[strum(serialize = "ATTi590750", to_string = "AttMsmqTransactional")]
    AttMsmqTransactional = 0x9039e,
    #[strum(serialize = "ATTk591161", to_string = "AttMsmqUserSid")]
    AttMsmqUserSid = 0x90539,
    #[strum(serialize = "ATTj590766", to_string = "AttMsmqVersion")]
    AttMsmqVersion = 0x903ae,
    #[strum(serialize = "ATTi590943", to_string = "AttMsnpallowdialin")]
    AttMsnpallowdialin = 0x9045f,
    #[strum(serialize = "ATTf590947", to_string = "AttMsnpcalledstationid")]
    AttMsnpcalledstationid = 0x90463,
    #[strum(serialize = "ATTf590948", to_string = "AttMsnpcallingstationid")]
    AttMsnpcallingstationid = 0x90464,
    #[strum(serialize = "ATTf590954", to_string = "AttMsnpsavedcallingstationid")]
    AttMsnpsavedcallingstationid = 0x9046a,
    #[strum(serialize = "ATTf590969", to_string = "AttMsradiuscallbacknumber")]
    AttMsradiuscallbacknumber = 0x90479,
    #[strum(serialize = "ATTj590977", to_string = "AttMsradiusframedipaddress")]
    AttMsradiusframedipaddress = 0x90481,
    #[strum(serialize = "ATTf590982", to_string = "AttMsradiusframedroute")]
    AttMsradiusframedroute = 0x90486,
    #[strum(serialize = "ATTj590995", to_string = "AttMsradiusservicetype")]
    AttMsradiusservicetype = 0x90493,
    #[strum(serialize = "ATTf591013", to_string = "AttMsrassavedcallbacknumber")]
    AttMsrassavedcallbacknumber = 0x904a5,
    #[strum(serialize = "ATTj591014", to_string = "AttMsrassavedframedipaddress")]
    AttMsrassavedframedipaddress = 0x904a6,
    #[strum(serialize = "ATTf591015", to_string = "AttMsrassavedframedroute")]
    AttMsrassavedframedroute = 0x904a7,
    #[strum(serialize = "ATTc131096", to_string = "AttMustContain")]
    AttMustContain = 0x20018,
    #[strum(serialize = "ATTj590577", to_string = "AttNameServiceFlags")]
    AttNameServiceFlags = 0x902f1,
    #[strum(serialize = "ATTb131088", to_string = "AttNcName")]
    AttNcName = 0x20010,
    #[strum(serialize = "ATTm589911", to_string = "AttNetbiosName")]
    AttNetbiosName = 0x90057,
    #[strum(serialize = "ATTi590673", to_string = "AttNetbootAllowNewClients")]
    AttNetbootAllowNewClients = 0x90351,
    #[strum(serialize = "ATTi590678", to_string = "AttNetbootAnswerOnlyValidClients")]
    AttNetbootAnswerOnlyValidClients = 0x90356,
    #[strum(serialize = "ATTi590677", to_string = "AttNetbootAnswerRequests")]
    AttNetbootAnswerRequests = 0x90355,
    #[strum(serialize = "ATTj590676", to_string = "AttNetbootCurrentClientCount")]
    AttNetbootCurrentClientCount = 0x90354,
    #[strum(serialize = "ATTk590183", to_string = "AttNetbootGuid")]
    AttNetbootGuid = 0x90167,
    #[strum(serialize = "ATTm590182", to_string = "AttNetbootInitialization")]
    AttNetbootInitialization = 0x90166,
    #[strum(serialize = "ATTm590681", to_string = "AttNetbootIntellimirrorOses")]
    AttNetbootIntellimirrorOses = 0x90359,
    #[strum(serialize = "ATTi590674", to_string = "AttNetbootLimitClients")]
    AttNetbootLimitClients = 0x90352,
    #[strum(serialize = "ATTm590683", to_string = "AttNetbootLocallyInstalledOses")]
    AttNetbootLocallyInstalledOses = 0x9035b,
    #[strum(serialize = "ATTm590185", to_string = "AttNetbootMachineFilePath")]
    AttNetbootMachineFilePath = 0x90169,
    #[strum(serialize = "ATTj590675", to_string = "AttNetbootMaxClients")]
    AttNetbootMaxClients = 0x90353,
    #[strum(serialize = "ATTm591065", to_string = "AttNetbootMirrorDataFile")]
    AttNetbootMirrorDataFile = 0x904d9,
    #[strum(serialize = "ATTm590679", to_string = "AttNetbootNewMachineNamingPolicy")]
    AttNetbootNewMachineNamingPolicy = 0x90357,
    #[strum(serialize = "ATTb590680", to_string = "AttNetbootNewMachineOu")]
    AttNetbootNewMachineOu = 0x90358,
    #[strum(serialize = "ATTb590688", to_string = "AttNetbootScpBl")]
    AttNetbootScpBl = 0x90360,
    #[strum(serialize = "ATTb590684", to_string = "AttNetbootServer")]
    AttNetbootServer = 0x9035c,
    #[strum(serialize = "ATTm591064", to_string = "AttNetbootSifFile")]
    AttNetbootSifFile = 0x904d8,
    #[strum(serialize = "ATTm590682", to_string = "AttNetbootTools")]
    AttNetbootTools = 0x9035a,
    #[strum(serialize = "ATTe131531", to_string = "AttNetworkAddress")]
    AttNetworkAddress = 0x201cb,
    #[strum(serialize = "ATTb590038", to_string = "AttNextLevelStore")]
    AttNextLevelStore = 0x900d6,
    #[strum(serialize = "ATTj589912", to_string = "AttNextRid")]
    AttNextRid = 0x90058,
    #[strum(serialize = "ATTb590354", to_string = "AttNonSecurityMember")]
    AttNonSecurityMember = 0x90212,
    #[strum(serialize = "ATTb590355", to_string = "AttNonSecurityMemberBl")]
    AttNonSecurityMemberBl = 0x90213,
    #[strum(serialize = "ATTb590127", to_string = "AttNotificationList")]
    AttNotificationList = 0x9012f,
    #[strum(serialize = "ATTk589913", to_string = "AttNtGroupMembers")]
    AttNtGroupMembers = 0x90059,
    #[strum(serialize = "ATTj590181", to_string = "AttNtMixedDomain")]
    AttNtMixedDomain = 0x90165,
    #[strum(serialize = "ATTk589918", to_string = "AttNtPwdHistory")]
    AttNtPwdHistory = 0x9005e,
    #[strum(serialize = "ATTp131353", to_string = "AttNtSecurityDescriptor")]
    AttNtSecurityDescriptor = 0x20119,
    #[strum(serialize = "ATTb49", to_string = "AttObjDistName")]
    AttObjDistName = 0x31,
    #[strum(serialize = "ATTb590606", to_string = "AttObjectCategory")]
    AttObjectCategory = 0x9030e,
    #[strum(serialize = "ATTc0", to_string = "AttObjectClass")]
    AttObjectClass = 0x0,
    #[strum(serialize = "ATTj131442", to_string = "AttObjectClassCategory")]
    AttObjectClassCategory = 0x20172,
    #[strum(serialize = "ATTm1572870", to_string = "AttObjectClasses")]
    AttObjectClasses = 0x180006,
    #[strum(serialize = "ATTj590330", to_string = "AttObjectCount")]
    AttObjectCount = 0x901fa,
    #[strum(serialize = "ATTk589826", to_string = "AttObjectGuid")]
    AttObjectGuid = 0x90002,
    #[strum(serialize = "ATTr589970", to_string = "AttObjectSid")]
    AttObjectSid = 0x90092,
    #[strum(serialize = "ATTj131148", to_string = "AttObjectVersion")]
    AttObjectVersion = 0x2004c,
    #[strum(serialize = "ATTm589975", to_string = "AttOemInformation")]
    AttOemInformation = 0x90097,
    #[strum(serialize = "ATTk131290", to_string = "AttOmObjectClass")]
    AttOmObjectClass = 0x200da,
    #[strum(serialize = "ATTj131303", to_string = "AttOmSyntax")]
    AttOmSyntax = 0x200e7,
    #[strum(serialize = "ATTk590329", to_string = "AttOmtGuid")]
    AttOmtGuid = 0x901f9,
    #[strum(serialize = "ATTk590157", to_string = "AttOmtIndxGuid")]
    AttOmtIndxGuid = 0x9014d,
    #[strum(serialize = "ATTm590187", to_string = "AttOperatingSystem")]
    AttOperatingSystem = 0x9016b,
    #[strum(serialize = "ATTm590239", to_string = "AttOperatingSystemHotfix")]
    AttOperatingSystemHotfix = 0x9019f,
    #[strum(serialize = "ATTm590189", to_string = "AttOperatingSystemServicePack")]
    AttOperatingSystemServicePack = 0x9016d,
    #[strum(serialize = "ATTm590188", to_string = "AttOperatingSystemVersion")]
    AttOperatingSystemVersion = 0x9016c,
    #[strum(serialize = "ATTj589968", to_string = "AttOperatorCount")]
    AttOperatorCount = 0x90090,
    #[strum(serialize = "ATTm590536", to_string = "AttOptionDescription")]
    AttOptionDescription = 0x902c8,
    #[strum(serialize = "ATTj590131", to_string = "AttOptions")]
    AttOptions = 0x90133,
    #[strum(serialize = "ATTf590537", to_string = "AttOptionsLocation")]
    AttOptionsLocation = 0x902c9,
    #[strum(serialize = "ATTm10", to_string = "AttOrganizationName")]
    AttOrganizationName = 0xa,
    #[strum(serialize = "ATTm11", to_string = "AttOrganizationalUnitName")]
    AttOrganizationalUnitName = 0xb,
    #[strum(serialize = "ATTk131517", to_string = "AttOriginalDisplayTable")]
    AttOriginalDisplayTable = 0x201bd,
    #[strum(serialize = "ATTk131286", to_string = "AttOriginalDisplayTableMsdos")]
    AttOriginalDisplayTableMsdos = 0x200d6,
    #[strum(serialize = "ATTm589915", to_string = "AttOtherLoginWorkstations")]
    AttOtherLoginWorkstations = 0x9005b,
    #[strum(serialize = "ATTm590475", to_string = "AttOtherMailbox")]
    AttOtherMailbox = 0x9028b,
    #[strum(serialize = "ATTm1441826", to_string = "AttOtherName")]
    AttOtherName = 0x160022,
    #[strum(serialize = "ATTh591183", to_string = "AttOtherWellKnownObjects")]
    AttOtherWellKnownObjects = 0x9054f,
    #[strum(serialize = "ATTb32", to_string = "AttOwner")]
    AttOwner = 0x20,
    #[strum(serialize = "ATTj590151", to_string = "AttPackageFlags")]
    AttPackageFlags = 0x90147,
    #[strum(serialize = "ATTm590150", to_string = "AttPackageName")]
    AttPackageName = 0x90146,
    #[strum(serialize = "ATTj590148", to_string = "AttPackageType")]
    AttPackageType = 0x90144,
    #[strum(serialize = "ATTb590381", to_string = "AttParentCa")]
    AttParentCa = 0x9022d,
    #[strum(serialize = "ATTk590509", to_string = "AttParentCaCertificateChain")]
    AttParentCaCertificateChain = 0x902ad,
    #[strum(serialize = "ATTk591048", to_string = "AttParentGuid")]
    AttParentGuid = 0x904c8,
    #[strum(serialize = "ATTk590487", to_string = "AttPartialAttributeDeletionList")]
    AttPartialAttributeDeletionList = 0x90297,
    #[strum(serialize = "ATTk590464", to_string = "AttPartialAttributeSet")]
    AttPartialAttributeSet = 0x90280,
    #[strum(serialize = "ATTq590690", to_string = "AttPekKeyChangeInterval")]
    AttPekKeyChangeInterval = 0x90362,
    #[strum(serialize = "ATTk590689", to_string = "AttPekList")]
    AttPekList = 0x90361,
    #[strum(serialize = "ATTk590517", to_string = "AttPendingCaCertificates")]
    AttPendingCaCertificates = 0x902b5,
    #[strum(serialize = "ATTb590519", to_string = "AttPendingParentCa")]
    AttPendingParentCa = 0x902b7,
    #[strum(serialize = "ATTk131397", to_string = "AttPerMsgDialogDisplayTable")]
    AttPerMsgDialogDisplayTable = 0x20145,
    #[strum(serialize = "ATTk131398", to_string = "AttPerRecipDialogDisplayTable")]
    AttPerRecipDialogDisplayTable = 0x20146,
    #[strum(serialize = "ATTm131687", to_string = "AttPersonalTitle")]
    AttPersonalTitle = 0x20267,
    #[strum(serialize = "ATTm590470", to_string = "AttPhoneFaxOther")]
    AttPhoneFaxOther = 0x90286,
    #[strum(serialize = "ATTm131349", to_string = "AttPhoneHomeOther")]
    AttPhoneHomeOther = 0x20115,
    #[strum(serialize = "ATTm1376276", to_string = "AttPhoneHomePrimary")]
    AttPhoneHomePrimary = 0x150014,
    #[strum(serialize = "ATTm590546", to_string = "AttPhoneIpOther")]
    AttPhoneIpOther = 0x902d2,
    #[strum(serialize = "ATTm590545", to_string = "AttPhoneIpPrimary")]
    AttPhoneIpPrimary = 0x902d1,
    #[strum(serialize = "ATTm590473", to_string = "AttPhoneIsdnPrimary")]
    AttPhoneIsdnPrimary = 0x90289,
    #[strum(serialize = "ATTm590471", to_string = "AttPhoneMobileOther")]
    AttPhoneMobileOther = 0x90287,
    #[strum(serialize = "ATTm1376297", to_string = "AttPhoneMobilePrimary")]
    AttPhoneMobilePrimary = 0x150029,
    #[strum(serialize = "ATTm131090", to_string = "AttPhoneOfficeOther")]
    AttPhoneOfficeOther = 0x20012,
    #[strum(serialize = "ATTm131190", to_string = "AttPhonePagerOther")]
    AttPhonePagerOther = 0x20076,
    #[strum(serialize = "ATTm1376298", to_string = "AttPhonePagerPrimary")]
    AttPhonePagerPrimary = 0x15002a,
    #[strum(serialize = "ATTk1376263", to_string = "AttPhoto")]
    AttPhoto = 0x150007,
    #[strum(serialize = "ATTm19", to_string = "AttPhysicalDeliveryOfficeName")]
    AttPhysicalDeliveryOfficeName = 0x13,
    #[strum(serialize = "ATTb590338", to_string = "AttPhysicalLocationObject")]
    AttPhysicalLocationObject = 0x90202,
    #[strum(serialize = "ATTk1441827", to_string = "AttPicture")]
    AttPicture = 0x160023,
    #[strum(serialize = "ATTm591154", to_string = "AttPkiCriticalExtensions")]
    AttPkiCriticalExtensions = 0x90532,
    #[strum(serialize = "ATTm591158", to_string = "AttPkiDefaultCsps")]
    AttPkiDefaultCsps = 0x90536,
    #[strum(serialize = "ATTj591151", to_string = "AttPkiDefaultKeySpec")]
    AttPkiDefaultKeySpec = 0x9052f,
    #[strum(serialize = "ATTp591159", to_string = "AttPkiEnrollmentAccess")]
    AttPkiEnrollmentAccess = 0x90537,
    #[strum(serialize = "ATTk591155", to_string = "AttPkiExpirationPeriod")]
    AttPkiExpirationPeriod = 0x90533,
    #[strum(serialize = "ATTm591157", to_string = "AttPkiExtendedKeyUsage")]
    AttPkiExtendedKeyUsage = 0x90535,
    #[strum(serialize = "ATTk591152", to_string = "AttPkiKeyUsage")]
    AttPkiKeyUsage = 0x90530,
    #[strum(serialize = "ATTj591153", to_string = "AttPkiMaxIssuingDepth")]
    AttPkiMaxIssuingDepth = 0x90531,
    #[strum(serialize = "ATTk591156", to_string = "AttPkiOverlapPeriod")]
    AttPkiOverlapPeriod = 0x90534,
    #[strum(serialize = "ATTk590030", to_string = "AttPkt")]
    AttPkt = 0x900ce,
    #[strum(serialize = "ATTk590029", to_string = "AttPktGuid")]
    AttPktGuid = 0x900cd,
    #[strum(serialize = "ATTj590457", to_string = "AttPolicyReplicationFlags")]
    AttPolicyReplicationFlags = 0x90279,
    #[strum(serialize = "ATTm590052", to_string = "AttPortName")]
    AttPortName = 0x900e4,
    #[strum(serialize = "ATTc131080", to_string = "AttPossSuperiors")]
    AttPossSuperiors = 0x20008,
    #[strum(serialize = "ATTc590739", to_string = "AttPossibleInferiors")]
    AttPossibleInferiors = 0x90393,
    #[strum(serialize = "ATTm18", to_string = "AttPostOfficeBox")]
    AttPostOfficeBox = 0x12,
    #[strum(serialize = "ATTm16", to_string = "AttPostalAddress")]
    AttPostalAddress = 0x10,
    #[strum(serialize = "ATTm17", to_string = "AttPostalCode")]
    AttPostalCode = 0x11,
    #[strum(serialize = "ATTj28", to_string = "AttPreferredDeliveryMethod")]
    AttPreferredDeliveryMethod = 0x1c,
    #[strum(serialize = "ATTm1441831", to_string = "AttPreferredlanguage")]
    AttPreferredlanguage = 0x160027,
    #[strum(serialize = "ATTb589921", to_string = "AttPreferredOu")]
    AttPreferredOu = 0x90061,
    #[strum(serialize = "ATTk590362", to_string = "AttPrefixMap")]
    AttPrefixMap = 0x9021a,
    #[strum(serialize = "ATTn29", to_string = "AttPresentationAddress")]
    AttPresentationAddress = 0x1d,
    #[strum(serialize = "ATTk590516", to_string = "AttPreviousCaCertificates")]
    AttPreviousCaCertificates = 0x902b4,
    #[strum(serialize = "ATTb590518", to_string = "AttPreviousParentCa")]
    AttPreviousParentCa = 0x902b6,
    #[strum(serialize = "ATTj589922", to_string = "AttPrimaryGroupId")]
    AttPrimaryGroupId = 0x90062,
    #[strum(serialize = "ATTj591236", to_string = "AttPrimaryGroupToken")]
    AttPrimaryGroupToken = 0x90584,
    #[strum(serialize = "ATTj590071", to_string = "AttPrintAttributes")]
    AttPrintAttributes = 0x900f7,
    #[strum(serialize = "ATTm590061", to_string = "AttPrintBinNames")]
    AttPrintBinNames = 0x900ed,
    #[strum(serialize = "ATTi590066", to_string = "AttPrintCollate")]
    AttPrintCollate = 0x900f2,
    #[strum(serialize = "ATTi590067", to_string = "AttPrintColor")]
    AttPrintColor = 0x900f3,
    #[strum(serialize = "ATTi591135", to_string = "AttPrintDuplexSupported")]
    AttPrintDuplexSupported = 0x9051f,
    #[strum(serialize = "ATTj590058", to_string = "AttPrintEndTime")]
    AttPrintEndTime = 0x900ea,
    #[strum(serialize = "ATTm590059", to_string = "AttPrintFormName")]
    AttPrintFormName = 0x900eb,
    #[strum(serialize = "ATTi590099", to_string = "AttPrintKeepPrintedJobs")]
    AttPrintKeepPrintedJobs = 0x90113,
    #[strum(serialize = "ATTm590070", to_string = "AttPrintLanguage")]
    AttPrintLanguage = 0x900f6,
    #[strum(serialize = "ATTm590112", to_string = "AttPrintMacAddress")]
    AttPrintMacAddress = 0x90120,
    #[strum(serialize = "ATTj590065", to_string = "AttPrintMaxCopies")]
    AttPrintMaxCopies = 0x900f1,
    #[strum(serialize = "ATTj590062", to_string = "AttPrintMaxResolutionSupported")]
    AttPrintMaxResolutionSupported = 0x900ee,
    #[strum(serialize = "ATTj590101", to_string = "AttPrintMaxXExtent")]
    AttPrintMaxXExtent = 0x90115,
    #[strum(serialize = "ATTj590102", to_string = "AttPrintMaxYExtent")]
    AttPrintMaxYExtent = 0x90116,
    #[strum(serialize = "ATTm590113", to_string = "AttPrintMediaReady")]
    AttPrintMediaReady = 0x90121,
    #[strum(serialize = "ATTm590123", to_string = "AttPrintMediaSupported")]
    AttPrintMediaSupported = 0x9012b,
    #[strum(serialize = "ATTj590106", to_string = "AttPrintMemory")]
    AttPrintMemory = 0x9011a,
    #[strum(serialize = "ATTj590103", to_string = "AttPrintMinXExtent")]
    AttPrintMinXExtent = 0x90117,
    #[strum(serialize = "ATTj590104", to_string = "AttPrintMinYExtent")]
    AttPrintMinYExtent = 0x90118,
    #[strum(serialize = "ATTm590111", to_string = "AttPrintNetworkAddress")]
    AttPrintNetworkAddress = 0x9011f,
    #[strum(serialize = "ATTm590096", to_string = "AttPrintNotify")]
    AttPrintNotify = 0x90110,
    #[strum(serialize = "ATTj590114", to_string = "AttPrintNumberUp")]
    AttPrintNumberUp = 0x90122,
    #[strum(serialize = "ATTm590064", to_string = "AttPrintOrientationsSupported")]
    AttPrintOrientationsSupported = 0x900f0,
    #[strum(serialize = "ATTm590095", to_string = "AttPrintOwner")]
    AttPrintOwner = 0x9010f,
    #[strum(serialize = "ATTj590455", to_string = "AttPrintPagesPerMinute")]
    AttPrintPagesPerMinute = 0x90277,
    #[strum(serialize = "ATTj590109", to_string = "AttPrintRate")]
    AttPrintRate = 0x9011d,
    #[strum(serialize = "ATTm590110", to_string = "AttPrintRateUnit")]
    AttPrintRateUnit = 0x9011e,
    #[strum(serialize = "ATTm590054", to_string = "AttPrintSeparatorFile")]
    AttPrintSeparatorFile = 0x900e6,
    #[strum(serialize = "ATTm590094", to_string = "AttPrintShareName")]
    AttPrintShareName = 0x9010e,
    #[strum(serialize = "ATTm590098", to_string = "AttPrintSpooling")]
    AttPrintSpooling = 0x90112,
    #[strum(serialize = "ATTi590105", to_string = "AttPrintStaplingSupported")]
    AttPrintStaplingSupported = 0x90119,
    #[strum(serialize = "ATTj590057", to_string = "AttPrintStartTime")]
    AttPrintStartTime = 0x900e9,
    #[strum(serialize = "ATTm590097", to_string = "AttPrintStatus")]
    AttPrintStatus = 0x90111,
    #[strum(serialize = "ATTm590124", to_string = "AttPrinterName")]
    AttPrinterName = 0x9012c,
    #[strum(serialize = "ATTq589923", to_string = "AttPriorSetTime")]
    AttPriorSetTime = 0x90063,
    #[strum(serialize = "ATTk589924", to_string = "AttPriorValue")]
    AttPriorValue = 0x90064,
    #[strum(serialize = "ATTj590055", to_string = "AttPriority")]
    AttPriority = 0x900e7,
    #[strum(serialize = "ATTk589925", to_string = "AttPrivateKey")]
    AttPrivateKey = 0x90065,
    #[strum(serialize = "ATTj590460", to_string = "AttPrivilegeAttributes")]
    AttPrivilegeAttributes = 0x9027c,
    #[strum(serialize = "ATTm590458", to_string = "AttPrivilegeDisplayName")]
    AttPrivilegeDisplayName = 0x9027a,
    #[strum(serialize = "ATTb590461", to_string = "AttPrivilegeHolder")]
    AttPrivilegeHolder = 0x9027d,
    #[strum(serialize = "ATTq590459", to_string = "AttPrivilegeValue")]
    AttPrivilegeValue = 0x9027b,
    #[strum(serialize = "ATTk590642", to_string = "AttProductCode")]
    AttProductCode = 0x90332,
    #[strum(serialize = "ATTm589963", to_string = "AttProfilePath")]
    AttProfilePath = 0x9008b,
    #[strum(serialize = "ATTh591073", to_string = "AttProxiedObjectName")]
    AttProxiedObjectName = 0x904e1,
    #[strum(serialize = "ATTm131282", to_string = "AttProxyAddresses")]
    AttProxyAddresses = 0x200d2,
    #[strum(serialize = "ATTi131595", to_string = "AttProxyGenerationEnabled")]
    AttProxyGenerationEnabled = 0x2020b,
    #[strum(serialize = "ATTq589927", to_string = "AttProxyLifetime")]
    AttProxyLifetime = 0x90067,
    #[strum(serialize = "ATTk590244", to_string = "AttPublicKeyPolicy")]
    AttPublicKeyPolicy = 0x901a4,
    #[strum(serialize = "ATTm590710", to_string = "AttPurportedSearch")]
    AttPurportedSearch = 0x90376,
    #[strum(serialize = "ATTj589919", to_string = "AttPwdHistoryLength")]
    AttPwdHistoryLength = 0x9005f,
    #[strum(serialize = "ATTq589920", to_string = "AttPwdLastSet")]
    AttPwdLastSet = 0x90060,
    #[strum(serialize = "ATTj589917", to_string = "AttPwdProperties")]
    AttPwdProperties = 0x9005d,
    #[strum(serialize = "ATTj590282", to_string = "AttQualityOfService")]
    AttQualityOfService = 0x901ca,
    #[strum(serialize = "ATTm591179", to_string = "AttQueryFilter")]
    AttQueryFilter = 0x9054b,
    #[strum(serialize = "ATTb590432", to_string = "AttQueryPolicyBl")]
    AttQueryPolicyBl = 0x90260,
    #[strum(serialize = "ATTb590431", to_string = "AttQueryPolicyObject")]
    AttQueryPolicyObject = 0x9025f,
    #[strum(serialize = "ATTm590504", to_string = "AttQuerypoint")]
    AttQuerypoint = 0x902a8,
    #[strum(serialize = "ATTj131106", to_string = "AttRangeLower")]
    AttRangeLower = 0x20022,
    #[strum(serialize = "ATTj131107", to_string = "AttRangeUpper")]
    AttRangeUpper = 0x20023,
    #[strum(serialize = "ATTm589825", to_string = "AttRdn")]
    AttRdn = 0x90001,
    #[strum(serialize = "ATTc131098", to_string = "AttRdnAttId")]
    AttRdnAttId = 0x2001a,
    #[strum(serialize = "ATTk26", to_string = "AttRegisteredAddress")]
    AttRegisteredAddress = 0x1a,
    #[strum(serialize = "ATTm589929", to_string = "AttRemoteServerName")]
    AttRemoteServerName = 0x90069,
    #[strum(serialize = "ATTm589931", to_string = "AttRemoteSource")]
    AttRemoteSource = 0x9006b,
    #[strum(serialize = "ATTj589932", to_string = "AttRemoteSourceType")]
    AttRemoteSourceType = 0x9006c,
    #[strum(serialize = "ATTm590633", to_string = "AttRemoteStorageGuid")]
    AttRemoteStorageGuid = 0x90329,
    #[strum(serialize = "ATTk589827", to_string = "AttReplPropertyMetaData")]
    AttReplPropertyMetaData = 0x90003,
    #[strum(serialize = "ATTj590501", to_string = "AttReplTopologyStayOfExecution")]
    AttReplTopologyStayOfExecution = 0x902a5,
    #[strum(serialize = "ATTk589828", to_string = "AttReplUptodateVector")]
    AttReplUptodateVector = 0x90004,
    #[strum(serialize = "ATTm589933", to_string = "AttReplicaSource")]
    AttReplicaSource = 0x9006d,
    #[strum(serialize = "ATTb131508", to_string = "AttReports")]
    AttReports = 0x201b4,
    #[strum(serialize = "ATTj591160", to_string = "AttReplInterval")]
    AttReplInterval = 0x90538,
    #[strum(serialize = "ATTk131163", to_string = "AttRepsFrom")]
    AttRepsFrom = 0x2005b,
    #[strum(serialize = "ATTk131155", to_string = "AttRepsTo")]
    AttRepsTo = 0x20053,
    #[strum(serialize = "ATTk590145", to_string = "AttRequiredCategories")]
    AttRequiredCategories = 0x90141,
    #[strum(serialize = "ATTk590497", to_string = "AttRetiredReplDsaSignatures")]
    AttRetiredReplDsaSignatures = 0x902a1,
    #[strum(serialize = "ATTr591125", to_string = "AttTokenGroups")]
    AttTokenGroups = 0x90515,
    #[strum(serialize = "ATTr591242", to_string = "AttTokenGroupsGlobalAndUniversal")]
    AttTokenGroupsGlobalAndUniversal = 0x9058a,
    #[strum(serialize = "ATTr591127", to_string = "AttTokenGroupsNoGcAcceptable")]
    AttTokenGroupsNoGcAcceptable = 0x90517,
    #[strum(serialize = "ATTj589969", to_string = "AttRevision")]
    AttRevision = 0x90091,
    #[strum(serialize = "ATTj589977", to_string = "AttRid")]
    AttRid = 0x90099,
    #[strum(serialize = "ATTq590195", to_string = "AttRidAllocationPool")]
    AttRidAllocationPool = 0x90173,
    #[strum(serialize = "ATTq590194", to_string = "AttRidAvailablePool")]
    AttRidAvailablePool = 0x90172,
    #[strum(serialize = "ATTb590192", to_string = "AttRidManagerReference")]
    AttRidManagerReference = 0x90170,
    #[strum(serialize = "ATTj590198", to_string = "AttRidNextRid")]
    AttRidNextRid = 0x90176,
    #[strum(serialize = "ATTq590196", to_string = "AttRidPreviousAllocationPool")]
    AttRidPreviousAllocationPool = 0x90174,
    #[strum(serialize = "ATTb590493", to_string = "AttRidSetReferences")]
    AttRidSetReferences = 0x9029d,
    #[strum(serialize = "ATTq590197", to_string = "AttRidUsedPool")]
    AttRidUsedPool = 0x90175,
    #[strum(serialize = "ATTm590164", to_string = "AttRightsGuid")]
    AttRightsGuid = 0x90154,
    #[strum(serialize = "ATTb33", to_string = "AttRoleOccupant")]
    AttRoleOccupant = 0x21,
    #[strum(serialize = "ATTm1376262", to_string = "AttRoomnumber")]
    AttRoomnumber = 0x150006,
    #[strum(serialize = "ATTb590498", to_string = "AttRootTrust")]
    AttRootTrust = 0x902a2,
    #[strum(serialize = "ATTm590190", to_string = "AttRpcNsAnnotation")]
    AttRpcNsAnnotation = 0x9016e,
    #[strum(serialize = "ATTm589937", to_string = "AttRpcNsBindings")]
    AttRpcNsBindings = 0x90071,
    #[strum(serialize = "ATTm590191", to_string = "AttRpcNsCodeset")]
    AttRpcNsCodeset = 0x9016f,
    #[strum(serialize = "ATTj590578", to_string = "AttRpcNsEntryFlags")]
    AttRpcNsEntryFlags = 0x902f2,
    #[strum(serialize = "ATTm589938", to_string = "AttRpcNsGroup")]
    AttRpcNsGroup = 0x90072,
    #[strum(serialize = "ATTm589939", to_string = "AttRpcNsInterfaceId")]
    AttRpcNsInterfaceId = 0x90073,
    #[strum(serialize = "ATTm590136", to_string = "AttRpcNsObjectId")]
    AttRpcNsObjectId = 0x90138,
    #[strum(serialize = "ATTj589941", to_string = "AttRpcNsPriority")]
    AttRpcNsPriority = 0x90075,
    #[strum(serialize = "ATTm589942", to_string = "AttRpcNsProfileEntry")]
    AttRpcNsProfileEntry = 0x90076,
    #[strum(serialize = "ATTm590138", to_string = "AttRpcNsTransferSyntax")]
    AttRpcNsTransferSyntax = 0x9013a,
    #[strum(serialize = "ATTm590045", to_string = "AttSamAccountName")]
    AttSamAccountName = 0x900dd,
    #[strum(serialize = "ATTj590126", to_string = "AttSamAccountType")]
    AttSamAccountType = 0x9012e,
    #[strum(serialize = "ATTk590035", to_string = "AttSchedule")]
    AttSchedule = 0x900d3,
    #[strum(serialize = "ATTj589944", to_string = "AttSchemaFlagsEx")]
    AttSchemaFlagsEx = 0x90078,
    #[strum(serialize = "ATTk589972", to_string = "AttSchemaIdGuid")]
    AttSchemaIdGuid = 0x90094,
    #[strum(serialize = "ATTk591182", to_string = "AttSchemaInfo")]
    AttSchemaInfo = 0x9054e,
    #[strum(serialize = "ATTl590305", to_string = "AttSchemaUpdate")]
    AttSchemaUpdate = 0x901e1,
    #[strum(serialize = "ATTj131543", to_string = "AttSchemaVersion")]
    AttSchemaVersion = 0x201d7,
    #[strum(serialize = "ATTj591178", to_string = "AttScopeFlags")]
    AttScopeFlags = 0x9054a,
    #[strum(serialize = "ATTm589886", to_string = "AttScriptPath")]
    AttScriptPath = 0x9003e,
    #[strum(serialize = "ATTj591128", to_string = "AttSdRightsEffective")]
    AttSdRightsEffective = 0x90518,
    #[strum(serialize = "ATTj131406", to_string = "AttSearchFlags")]
    AttSearchFlags = 0x2014e,
    #[strum(serialize = "ATTk14", to_string = "AttSearchGuide")]
    AttSearchGuide = 0xe,
    #[strum(serialize = "ATTb1376277", to_string = "AttSecretary")]
    AttSecretary = 0x150015,
    #[strum(serialize = "ATTr589945", to_string = "AttSecurityIdentifier")]
    AttSecurityIdentifier = 0x90079,
    #[strum(serialize = "ATTb34", to_string = "AttSeeAlso")]
    AttSeeAlso = 0x22,
    #[strum(serialize = "ATTj590328", to_string = "AttSeqNotification")]
    AttSeqNotification = 0x901f8,
    #[strum(serialize = "ATTf5", to_string = "AttSerialNumber")]
    AttSerialNumber = 0x5,
    #[strum(serialize = "ATTm590047", to_string = "AttServerName")]
    AttServerName = 0x900df,
    #[strum(serialize = "ATTb590339", to_string = "AttServerReference")]
    AttServerReference = 0x90203,
    #[strum(serialize = "ATTb590340", to_string = "AttServerReferenceBl")]
    AttServerReferenceBl = 0x90204,
    #[strum(serialize = "ATTj589981", to_string = "AttServerRole")]
    AttServerRole = 0x9009d,
    #[strum(serialize = "ATTj589978", to_string = "AttServerState")]
    AttServerState = 0x9009a,
    #[strum(serialize = "ATTm590334", to_string = "AttServiceBindingInformation")]
    AttServiceBindingInformation = 0x901fe,
    #[strum(serialize = "ATTk589946", to_string = "AttServiceClassId")]
    AttServiceClassId = 0x9007a,
    #[strum(serialize = "ATTk589947", to_string = "AttServiceClassInfo")]
    AttServiceClassInfo = 0x9007b,
    #[strum(serialize = "ATTm590333", to_string = "AttServiceClassName")]
    AttServiceClassName = 0x901fd,
    #[strum(serialize = "ATTm590481", to_string = "AttServiceDnsName")]
    AttServiceDnsName = 0x90291,
    #[strum(serialize = "ATTm590483", to_string = "AttServiceDnsNameType")]
    AttServiceDnsNameType = 0x90293,
    #[strum(serialize = "ATTk590023", to_string = "AttServiceInstanceVersion")]
    AttServiceInstanceVersion = 0x900c7,
    #[strum(serialize = "ATTm590595", to_string = "AttServicePrincipalName")]
    AttServicePrincipalName = 0x90303,
    #[strum(serialize = "ATTm590149", to_string = "AttSetupCommand")]
    AttSetupCommand = 0x90145,
    #[strum(serialize = "ATTm590439", to_string = "AttShellContextMenu")]
    AttShellContextMenu = 0x90267,
    #[strum(serialize = "ATTm590387", to_string = "AttShellPropertyPages")]
    AttShellPropertyPages = 0x90233,
    #[strum(serialize = "ATTm591033", to_string = "AttShortServerName")]
    AttShortServerName = 0x904b9,
    #[strum(serialize = "ATTb590468", to_string = "AttShowInAddressBook")]
    AttShowInAddressBook = 0x90284,
    #[strum(serialize = "ATTi131241", to_string = "AttShowInAdvancedViewOnly")]
    AttShowInAdvancedViewOnly = 0x200a9,
    #[strum(serialize = "ATTr590433", to_string = "AttSidHistory")]
    AttSidHistory = 0x90261,
    #[strum(serialize = "ATTm590648", to_string = "AttSignatureAlgorithms")]
    AttSignatureAlgorithms = 0x90338,
    #[strum(serialize = "ATTk590186", to_string = "AttSiteGuid")]
    AttSiteGuid = 0x9016a,
    #[strum(serialize = "ATTb590646", to_string = "AttSiteLinkList")]
    AttSiteLinkList = 0x90336,
    #[strum(serialize = "ATTb590645", to_string = "AttSiteList")]
    AttSiteList = 0x90335,
    #[strum(serialize = "ATTb590336", to_string = "AttSiteObject")]
    AttSiteObject = 0x90200,
    #[strum(serialize = "ATTb590337", to_string = "AttSiteObjectBl")]
    AttSiteObjectBl = 0x90201,
    #[strum(serialize = "ATTb590318", to_string = "AttSiteServer")]
    AttSiteServer = 0x901ee,
    #[strum(serialize = "ATTm590610", to_string = "AttSmtpMailAddress")]
    AttSmtpMailAddress = 0x90312,
    #[strum(serialize = "ATTm591171", to_string = "AttSpnMappings")]
    AttSpnMappings = 0x90543,
    #[strum(serialize = "ATTm8", to_string = "AttStateOrProvinceName")]
    AttStateOrProvinceName = 0x8,
    #[strum(serialize = "ATTm9", to_string = "AttStreetAddress")]
    AttStreetAddress = 0x9,
    #[strum(serialize = "ATTc1572873", to_string = "AttStructuralObjectClass")]
    AttStructuralObjectClass = 0x180009,
    #[strum(serialize = "ATTc131093", to_string = "AttSubClassOf")]
    AttSubClassOf = 0x20015,
    #[strum(serialize = "ATTb131079", to_string = "AttSubRefs")]
    AttSubRefs = 0x20007,
    #[strum(serialize = "ATTb1638410", to_string = "AttSubschemasubentry")]
    AttSubschemasubentry = 0x19000a,
    #[strum(serialize = "ATTm590535", to_string = "AttSuperScopeDescription")]
    AttSuperScopeDescription = 0x902c7,
    #[strum(serialize = "ATTf590534", to_string = "AttSuperScopes")]
    AttSuperScopes = 0x902c6,
    #[strum(serialize = "ATTm590356", to_string = "AttSuperiorDnsRoot")]
    AttSuperiorDnsRoot = 0x90214,
    #[strum(serialize = "ATTk589949", to_string = "AttSupplementalCredentials")]
    AttSupplementalCredentials = 0x9007d,
    #[strum(serialize = "ATTk30", to_string = "AttSupportedApplicationContext")]
    AttSupportedApplicationContext = 0x1e,
    #[strum(serialize = "ATTm4", to_string = "AttSurname")]
    AttSurname = 0x4,
    #[strum(serialize = "ATTj590490", to_string = "AttSyncAttributes")]
    AttSyncAttributes = 0x9029a,
    #[strum(serialize = "ATTb590489", to_string = "AttSyncMembership")]
    AttSyncMembership = 0x90299,
    #[strum(serialize = "ATTb590488", to_string = "AttSyncWithObject")]
    AttSyncWithObject = 0x90298,
    #[strum(serialize = "ATTr590491", to_string = "AttSyncWithSid")]
    AttSyncWithSid = 0x9029b,
    #[strum(serialize = "ATTc590022", to_string = "AttSystemAuxiliaryClass")]
    AttSystemAuxiliaryClass = 0x900c6,
    #[strum(serialize = "ATTj590199", to_string = "AttSystemFlags")]
    AttSystemFlags = 0x90177,
    #[strum(serialize = "ATTc590020", to_string = "AttSystemMayContain")]
    AttSystemMayContain = 0x900c4,
    #[strum(serialize = "ATTc590021", to_string = "AttSystemMustContain")]
    AttSystemMustContain = 0x900c5,
    #[strum(serialize = "ATTi589994", to_string = "AttSystemOnly")]
    AttSystemOnly = 0x900aa,
    #[strum(serialize = "ATTc590019", to_string = "AttSystemPossSuperiors")]
    AttSystemPossSuperiors = 0x900c3,
    #[strum(serialize = "ATTm20", to_string = "AttTelephoneNumber")]
    AttTelephoneNumber = 0x14,
    #[strum(serialize = "ATTk22", to_string = "AttTeletexTerminalIdentifier")]
    AttTeletexTerminalIdentifier = 0x16,
    #[strum(serialize = "ATTk21", to_string = "AttTelexNumber")]
    AttTelexNumber = 0x15,
    #[strum(serialize = "ATTm590472", to_string = "AttTelexPrimary")]
    AttTelexPrimary = 0x90288,
    #[strum(serialize = "ATTb591170", to_string = "AttTemplateRoots")]
    AttTemplateRoots = 0x90542,
    #[strum(serialize = "ATTk590709", to_string = "AttTerminalServer")]
    AttTerminalServer = 0x90375,
    #[strum(serialize = "ATTm131203", to_string = "AttTextCountry")]
    AttTextCountry = 0x20083,
    #[strum(serialize = "ATTm1376258", to_string = "AttTextEncodedOrAddress")]
    AttTextEncodedOrAddress = 0x150002,
    #[strum(serialize = "ATTq590327", to_string = "AttTimeRefresh")]
    AttTimeRefresh = 0x901f7,
    #[strum(serialize = "ATTq590326", to_string = "AttTimeVolChange")]
    AttTimeVolChange = 0x901f6,
    #[strum(serialize = "ATTm12", to_string = "AttTitle")]
    AttTitle = 0xc,
    #[strum(serialize = "ATTj131126", to_string = "AttTombstoneLifetime")]
    AttTombstoneLifetime = 0x20036,
    #[strum(serialize = "ATTc590719", to_string = "AttTransportAddressAttribute")]
    AttTransportAddressAttribute = 0x9037f,
    #[strum(serialize = "ATTm590613", to_string = "AttTransportDllName")]
    AttTransportDllName = 0x90315,
    #[strum(serialize = "ATTb590615", to_string = "AttTransportType")]
    AttTransportType = 0x90317,
    #[strum(serialize = "ATTi590630", to_string = "AttTreatAsLeaf")]
    AttTreatAsLeaf = 0x90326,
    #[strum(serialize = "ATTm590484", to_string = "AttTreeName")]
    AttTreeName = 0x90294,
    #[strum(serialize = "ATTj590294", to_string = "AttTrustAttributes")]
    AttTrustAttributes = 0x901d6,
    #[strum(serialize = "ATTk589953", to_string = "AttTrustAuthIncoming")]
    AttTrustAuthIncoming = 0x90081,
    #[strum(serialize = "ATTk589959", to_string = "AttTrustAuthOutgoing")]
    AttTrustAuthOutgoing = 0x90087,
    #[strum(serialize = "ATTj589956", to_string = "AttTrustDirection")]
    AttTrustDirection = 0x90084,
    #[strum(serialize = "ATTb590295", to_string = "AttTrustParent")]
    AttTrustParent = 0x901d7,
    #[strum(serialize = "ATTm589957", to_string = "AttTrustPartner")]
    AttTrustPartner = 0x90085,
    #[strum(serialize = "ATTj589958", to_string = "AttTrustPosixOffset")]
    AttTrustPosixOffset = 0x90086,
    #[strum(serialize = "ATTj589960", to_string = "AttTrustType")]
    AttTrustType = 0x90088,
    #[strum(serialize = "ATTj589979", to_string = "AttUasCompat")]
    AttUasCompat = 0x9009b,
    #[strum(serialize = "ATTm1376257", to_string = "AttUid")]
    AttUid = 0x150001,
    #[strum(serialize = "ATTm589961", to_string = "AttUncName")]
    AttUncName = 0x90089,
    #[strum(serialize = "ATTk589914", to_string = "AttUnicodePwd")]
    AttUnicodePwd = 0x9005a,
    #[strum(serialize = "ATTb50", to_string = "AttUniquemember")]
    AttUniquemember = 0x32,
    #[strum(serialize = "ATTk590637", to_string = "AttUpgradeProductCode")]
    AttUpgradeProductCode = 0x9032d,
    #[strum(serialize = "ATTm590714", to_string = "AttUpnSuffixes")]
    AttUpnSuffixes = 0x9037a,
    #[strum(serialize = "ATTj589832", to_string = "AttUserAccountControl")]
    AttUserAccountControl = 0x90008,
    #[strum(serialize = "ATTk590469", to_string = "AttUserCert")]
    AttUserCert = 0x90285,
    #[strum(serialize = "ATTm589980", to_string = "AttUserComment")]
    AttUserComment = 0x9009c,
    #[strum(serialize = "ATTm589962", to_string = "AttUserParameters")]
    AttUserParameters = 0x9008a,
    #[strum(serialize = "ATTk35", to_string = "AttUserPassword")]
    AttUserPassword = 0x23,
    #[strum(serialize = "ATTk1442008", to_string = "AttUserpkcs12")]
    AttUserpkcs12 = 0x1600d8,
    #[strum(serialize = "ATTm590480", to_string = "AttUserPrincipalName")]
    AttUserPrincipalName = 0x90290,
    #[strum(serialize = "ATTm590575", to_string = "AttUserSharedFolder")]
    AttUserSharedFolder = 0x902ef,
    #[strum(serialize = "ATTm590576", to_string = "AttUserSharedFolderOther")]
    AttUserSharedFolderOther = 0x902f0,
    #[strum(serialize = "ATTk1310860", to_string = "AttUserSmimeCertificate")]
    AttUserSmimeCertificate = 0x14008c,
    #[strum(serialize = "ATTm589910", to_string = "AttUserWorkstations")]
    AttUserWorkstations = 0x90056,
    #[strum(serialize = "ATTq131192", to_string = "AttUsnChanged")]
    AttUsnChanged = 0x20078,
    #[strum(serialize = "ATTq131091", to_string = "AttUsnCreated")]
    AttUsnCreated = 0x20013,
    #[strum(serialize = "ATTq131339", to_string = "AttUsnDsaLastObjRemoved")]
    AttUsnDsaLastObjRemoved = 0x2010b,
    #[strum(serialize = "ATTj131541", to_string = "AttUsnIntersite")]
    AttUsnIntersite = 0x201d5,
    #[strum(serialize = "ATTq131193", to_string = "AttUsnLastObjRem")]
    AttUsnLastObjRem = 0x20079,
    #[strum(serialize = "ATTq590720", to_string = "AttUsnSource")]
    AttUsnSource = 0x90380,
    #[strum(serialize = "ATTj591180", to_string = "AttValidAccesses")]
    AttValidAccesses = 0x9054c,
    #[strum(serialize = "ATTm590079", to_string = "AttVendor")]
    AttVendor = 0x900ff,
    #[strum(serialize = "ATTj589965", to_string = "AttVersionNumber")]
    AttVersionNumber = 0x9008d,
    #[strum(serialize = "ATTj590152", to_string = "AttVersionNumberHi")]
    AttVersionNumberHi = 0x90148,
    #[strum(serialize = "ATTj590153", to_string = "AttVersionNumberLo")]
    AttVersionNumberLo = 0x90149,
    #[strum(serialize = "ATTk590160", to_string = "AttVolTableGuid")]
    AttVolTableGuid = 0x90150,
    #[strum(serialize = "ATTk590158", to_string = "AttVolTableIdxGuid")]
    AttVolTableIdxGuid = 0x9014e,
    #[strum(serialize = "ATTj590331", to_string = "AttVolumeCount")]
    AttVolumeCount = 0x901fb,
    #[strum(serialize = "ATTm590125", to_string = "AttWbemPath")]
    AttWbemPath = 0x9012d,
    #[strum(serialize = "ATTh590442", to_string = "AttWellKnownObjects")]
    AttWellKnownObjects = 0x9026a,
    #[strum(serialize = "ATTl131075", to_string = "AttWhenChanged")]
    AttWhenChanged = 0x20003,
    #[strum(serialize = "ATTl131074", to_string = "AttWhenCreated")]
    AttWhenCreated = 0x20002,
    #[strum(serialize = "ATTk589966", to_string = "AttWinsockAddresses")]
    AttWinsockAddresses = 0x9008e,
    #[strum(serialize = "ATTm131536", to_string = "AttWwwHomePage")]
    AttWwwHomePage = 0x201d0,
    #[strum(serialize = "ATTm590573", to_string = "AttWwwPageOther")]
    AttWwwPageOther = 0x902ed,
    #[strum(serialize = "ATTg24", to_string = "AttX121Address")]
    AttX121Address = 0x18,
    #[strum(serialize = "ATTk45", to_string = "AttX500Uniqueidentifier")]
    AttX500Uniqueidentifier = 0x2d,
    #[strum(serialize = "ATTk36", to_string = "AttX509Cert")]
    AttX509Cert = 0x24,
    #[strum(serialize = "DNT_col", to_string = "DsRecordId")]
    DsRecordId = 0x7fffff01,
    #[strum(serialize = "PDNT_col", to_string = "DsParentRecordId")]
    DsParentRecordId = 0x7fffff02,
    #[strum(serialize = "time_col", to_string = "DsRecordTime")]
    DsRecordTime = 0x7fffff03,
    #[strum(serialize = "Ancestors_col", to_string = "DsAncestors")]
    DsAncestors = 0x7fffff04,
}
