#![allow(warnings, clippy::if_same_then_else)]
use crate::{sealed::Sealed, StandardWallIndicator::*, TransitionDay::*, UtcLocalIndicator::*, *};
const AFRICA_ALGIERS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2486592732, -1855958961, -1689814800, -1680397200, -1665363600, -1648342800,
			-1635123600, -1616893200, -1604278800, -1585443600, -1574038800, -1552266000,
			-1539997200, -1531443600, -956365200, -950486400, -942012000, -812502000, -796262400,
			-781052400, -766630800, -733280400, -439430400, -212029200, 41468400, 54774000,
			231724800, 246236400, 259545600, 275274000, 309740400, 325468800, 341802000, 357523200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 5, 4, 5, 4, 5, 3, 5, 3, 2, 3, 2, 5, 4,
			5, 3, 2, 3, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 732, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 561, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 22 },
		],
		time_zone_designations: &["LMT", "PMT", "WEST", "WET", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: None,
	},
};
const AFRICA_BISSAU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1830380400, 157770000],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -3740, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-01", "GMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "GMT", offset: 0 }, dst_info: None },
};
const AFRICA_CASABLANCA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1773012580, -956361600, -950490000, -942019200, -761187600, -617241600, -605149200,
			-81432000, -71110800, 141264000, 147222000, 199756800, 207702000, 231292800, 244249200,
			265507200, 271033200, 448243200, 504918000, 1212278400, 1220223600, 1243814400,
			1250809200, 1272758400, 1281222000, 1301788800, 1312066800, 1335664800, 1342749600,
			1345428000, 1348970400, 1367114400, 1373162400, 1376100000, 1382839200, 1396144800,
			1403920800, 1406944800, 1414288800, 1427594400, 1434247200, 1437271200, 1445738400,
			1459044000, 1465092000, 1468116000, 1477792800, 1490493600, 1495332000, 1498960800,
			1509242400, 1521943200, 1526176800, 1529200800, 1540692000, 1557021600, 1560045600,
			1587261600, 1590890400, 1618106400, 1621130400, 1648346400, 1651975200, 1679191200,
			1682215200, 1710036000, 1713060000, 1740276000, 1743904800, 1771120800, 1774144800,
			1801965600, 1804989600, 1832205600, 1835834400, 1863050400, 1866074400, 1893290400,
			1896919200, 1924135200, 1927159200, 1954980000, 1958004000, 1985220000, 1988848800,
			2016064800, 2019088800, 2046304800, 2049933600, 2077149600, 2080778400, 2107994400,
			2111018400, 2138234400, 2141863200, 2169079200, 2172103200, 2199924000, 2202948000,
			2230164000, 2233792800, 2261008800, 2264032800, 2291248800, 2294877600, 2322093600,
			2325722400, 2352938400, 2355962400, 2383178400, 2386807200, 2414023200, 2417047200,
			2444868000, 2447892000, 2475108000, 2478736800, 2505952800, 2508976800, 2536192800,
			2539821600, 2567037600, 2570666400, 2597882400, 2600906400, 2628122400, 2631751200,
			2658967200, 2661991200, 2689812000, 2692836000, 2720052000, 2723680800, 2750896800,
			2753920800, 2781136800, 2784765600, 2811981600, 2815610400, 2842826400, 2845850400,
			2873066400, 2876695200, 2903911200, 2906935200, 2934756000, 2937780000, 2964996000,
			2968624800, 2995840800, 2998864800, 3026080800, 3029709600, 3056925600, 3060554400,
			3087770400, 3090794400, 3118010400, 3121639200, 3148855200, 3151879200, 3179700000,
			3182724000, 3209940000, 3213568800, 3240784800, 3243808800, 3271024800, 3274653600,
			3301869600, 3305498400, 3332714400, 3335738400, 3362954400, 3366583200, 3393799200,
			3396823200, 3424644000, 3427668000, 3454884000, 3458512800, 3485728800, 3488752800,
			3515968800, 3519597600, 3546813600, 3549837600, 3577658400, 3580682400, 3607898400,
			3611527200, 3638743200, 3641767200, 3669588000, 3672612000, 3699828000, 3703456800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -1820, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "+01", "+00", "+01", "+00"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+01", offset: -3600 },
		dst_info: None,
	},
};
const AFRICA_CEUTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2177452800, -1630112400, -1616810400, -1442451600, -1427673600, -1379293200,
			-1364774400, -1348448400, -1333324800, -1316390400, -1301270400, -81432000, -71110800,
			141264000, 147222000, 199756800, 207702000, 231292800, 244249200, 265507200, 271033200,
			448243200, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400, 606877200,
			622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600, 733280400,
			749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -1276, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 17 },
		],
		time_zone_designations: &["LMT", "WET", "WEST", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const AFRICA_EL_AAIUN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1136070432, 198291600, 199756800, 207702000, 231292800, 244249200, 265507200,
			271033200, 1212278400, 1220223600, 1243814400, 1250809200, 1272758400, 1281222000,
			1301788800, 1312066800, 1335664800, 1342749600, 1345428000, 1348970400, 1367114400,
			1373162400, 1376100000, 1382839200, 1396144800, 1403920800, 1406944800, 1414288800,
			1427594400, 1434247200, 1437271200, 1445738400, 1459044000, 1465092000, 1468116000,
			1477792800, 1490493600, 1495332000, 1498960800, 1509242400, 1521943200, 1526176800,
			1529200800, 1540692000, 1557021600, 1560045600, 1587261600, 1590890400, 1618106400,
			1621130400, 1648346400, 1651975200, 1679191200, 1682215200, 1710036000, 1713060000,
			1740276000, 1743904800, 1771120800, 1774144800, 1801965600, 1804989600, 1832205600,
			1835834400, 1863050400, 1866074400, 1893290400, 1896919200, 1924135200, 1927159200,
			1954980000, 1958004000, 1985220000, 1988848800, 2016064800, 2019088800, 2046304800,
			2049933600, 2077149600, 2080778400, 2107994400, 2111018400, 2138234400, 2141863200,
			2169079200, 2172103200, 2199924000, 2202948000, 2230164000, 2233792800, 2261008800,
			2264032800, 2291248800, 2294877600, 2322093600, 2325722400, 2352938400, 2355962400,
			2383178400, 2386807200, 2414023200, 2417047200, 2444868000, 2447892000, 2475108000,
			2478736800, 2505952800, 2508976800, 2536192800, 2539821600, 2567037600, 2570666400,
			2597882400, 2600906400, 2628122400, 2631751200, 2658967200, 2661991200, 2689812000,
			2692836000, 2720052000, 2723680800, 2750896800, 2753920800, 2781136800, 2784765600,
			2811981600, 2815610400, 2842826400, 2845850400, 2873066400, 2876695200, 2903911200,
			2906935200, 2934756000, 2937780000, 2964996000, 2968624800, 2995840800, 2998864800,
			3026080800, 3029709600, 3056925600, 3060554400, 3087770400, 3090794400, 3118010400,
			3121639200, 3148855200, 3151879200, 3179700000, 3182724000, 3209940000, 3213568800,
			3240784800, 3243808800, 3271024800, 3274653600, 3301869600, 3305498400, 3332714400,
			3335738400, 3362954400, 3366583200, 3393799200, 3396823200, 3424644000, 3427668000,
			3454884000, 3458512800, 3485728800, 3488752800, 3515968800, 3519597600, 3546813600,
			3549837600, 3577658400, 3580682400, 3607898400, 3611527200, 3638743200, 3641767200,
			3669588000, 3672612000, 3699828000, 3703456800,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -3168, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-01", "+01", "+00", "+00", "+01"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+01", offset: -3600 },
		dst_info: None,
	},
};
const AFRICA_JUBA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1230775588, 10360800, 24786000, 41810400, 56322000, 73432800, 87944400, 104882400,
			119480400, 136332000, 151016400, 167781600, 182552400, 199231200, 214174800, 230680800,
			245710800, 262735200, 277246800, 294184800, 308782800, 325634400, 340405200, 357084000,
			371941200, 388533600, 403477200, 419983200, 435013200, 452037600, 466635600, 483487200,
			498171600, 947930400, 1612126800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 7588, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "CAST", "CAT", "EAT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CAT", offset: -7200 },
		dst_info: None,
	},
};
const AFRICA_KHARTOUM: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1230775808, 10360800, 24786000, 41810400, 56322000, 73432800, 87944400, 104882400,
			119480400, 136332000, 151016400, 167781600, 182552400, 199231200, 214174800, 230680800,
			245710800, 262735200, 277246800, 294184800, 308782800, 325634400, 340405200, 357084000,
			371941200, 388533600, 403477200, 419983200, 435013200, 452037600, 466635600, 483487200,
			498171600, 947930400, 1509483600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 7808, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "CAST", "CAT", "EAT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CAT", offset: -7200 },
		dst_info: None,
	},
};
const AFRICA_LUSAKA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2109291020],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 7820, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "CAT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CAT", offset: -7200 },
		dst_info: None,
	},
};
const AFRICA_MBABANE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2458173120, -2109288600, -860976000, -845254800, -829526400, -813805200,
		],
		transition_types: &[1, 3, 2, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 6720, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "SAST", "SAST", "SAST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "SAST", offset: -7200 },
		dst_info: None,
	},
};
const AFRICA_MONROVIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2776979812, -1604359012, 63593070],
		transition_types: &[1, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -2588, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -2588, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -2670, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "MMT", "MMT", "GMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "GMT", offset: 0 }, dst_info: None },
};
const AFRICA_NDJAMENA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1830387612, 308703600, 321314400],
		transition_types: &[1, 2, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3612, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "WAT", "WAST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WAT", offset: -3600 },
		dst_info: None,
	},
};
const AFRICA_PORTO_NOVO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2035584815, -1940889600, -1767226415, -1588465800],
		transition_types: &[1, 0, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 815, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 1800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "GMT", "+0030", "WAT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WAT", offset: -3600 },
		dst_info: None,
	},
};
const AFRICA_SAO_TOME: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2713912016, -1830384000, 1514768400, 1546304400],
		transition_types: &[1, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 1616, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -2205, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "LMT", "GMT", "WAT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "GMT", offset: 0 }, dst_info: None },
};
const AFRICA_TUNIS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2797202444, -1855958961, -969242400, -950493600, -941940000, -891136800, -877827600,
			-857257200, -844556400, -842918400, -842223600, -828230400, -812502000, -796269600,
			-781052400, -766634400, 231202800, 243903600, 262825200, 276044400, 581122800,
			591145200, 606870000, 622594800, 641516400, 654649200, 1114902000, 1128038400,
			1143334800, 1162083600, 1174784400, 1193533200, 1206838800, 1224982800,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 2444, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 561, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "PMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: None,
	},
};
const AFRICA_WINDHOEK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2458170504, -2109288600, -860976000, -845254800, 637970400, 764200800, 778640400,
			796780800, 810090000, 828835200, 841539600, 860284800, 873594000, 891734400, 905043600,
			923184000, 936493200, 954633600, 967942800, 986083200, 999392400, 1018137600,
			1030842000, 1049587200, 1062896400, 1081036800, 1094346000, 1112486400, 1125795600,
			1143936000, 1157245200, 1175385600, 1188694800, 1207440000, 1220749200, 1238889600,
			1252198800, 1270339200, 1283648400, 1301788800, 1315098000, 1333238400, 1346547600,
			1365292800, 1377997200, 1396742400, 1410051600, 1428192000, 1441501200, 1459641600,
			1472950800, 1491091200, 1504400400,
		],
		transition_types: &[
			1, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 4104, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 15 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 19 },
		],
		time_zone_designations: &["LMT", "+0130", "SAST", "SAST", "WAT", "CAT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CAT", offset: -7200 },
		dst_info: None,
	},
};
const AMERICA_ARAGUAINA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767214032, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-191365200, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600, 592974000, 602042400, 624423600, 634701600, 813726000, 824004000, 844570800,
			856058400, 876106800, 888717600, 908074800, 919562400, 938919600, 951616800, 970974000,
			982461600, 1003028400, 1013911200, 1036292400, 1045360800, 1350788400, 1361066400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -11568, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_LA_RIOJA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372095956, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667792800, 673588800, 687927600, 699415200, 719377200,
			731469600, 938919600, 952052400, 1086058800, 1087704000, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 2, 5, 4, 5, 4, 5, 3, 5,
			2, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16044, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_RIO_GALLEGOS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372095388, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687927600, 699415200, 719377200, 731469600,
			938919600, 952052400, 1086058800, 1087704000, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 3, 5, 2,
			5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16612, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_SALTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372096300, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687931200, 699415200, 719377200, 731469600,
			938919600, 952052400, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 2, 4, 5, 4, 5, 3, 5, 4,
			5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15700, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_SAN_JUAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372095556, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667792800, 673588800, 687927600, 699415200, 719377200,
			731469600, 938919600, 952052400, 1085972400, 1090728000, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 2, 5, 4, 5, 4, 5, 3, 5,
			2, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16444, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_SAN_LUIS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372096076, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 637380000, 655963200, 667796400, 675748800, 938919600, 952052400,
			1085972400, 1090728000, 1198983600, 1200880800, 1205031600, 1223784000, 1236481200,
			1255233600,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 2, 3, 2, 5, 3, 5, 2, 5, 4, 3,
			2, 3, 2, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15924, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_TUCUMAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372096348, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687931200, 699415200, 719377200, 731469600,
			938919600, 952052400, 1086058800, 1087099200, 1198983600, 1205632800, 1224385200,
			1237082400,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 2, 4, 5, 4, 5, 3, 5, 2,
			5, 4, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15652, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ARGENTINA_USHUAIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372095608, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687927600, 699415200, 719377200, 731469600,
			938919600, 952052400, 1085886000, 1087704000, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 3, 5, 2,
			5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16392, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_ASUNCION: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524507760, -1206389360, 86760000, 134017200, 181368000, 194497200, 212990400,
			226033200, 244526400, 257569200, 276062400, 291783600, 307598400, 323406000, 339220800,
			354942000, 370756800, 386478000, 402292800, 418014000, 433828800, 449636400, 465451200,
			481172400, 496987200, 512708400, 528523200, 544244400, 560059200, 575866800, 591681600,
			607402800, 625032000, 638938800, 654753600, 670474800, 686721600, 699418800, 718257600,
			733546800, 749448000, 762318000, 780984000, 793767600, 812520000, 825649200, 844574400,
			856666800, 876024000, 888721200, 907473600, 920775600, 938923200, 952225200, 970372800,
			983674800, 1002427200, 1018148400, 1030852800, 1049598000, 1062907200, 1081047600,
			1097985600, 1110682800, 1129435200, 1142132400, 1160884800, 1173582000, 1192939200,
			1205031600, 1224388800, 1236481200, 1255838400, 1270954800, 1286078400, 1302404400,
			1317528000, 1333854000, 1349582400,
		],
		transition_types: &[
			1, 2, 3, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4,
			2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2,
			4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13840, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -13840, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "AMT", "-04", "-03", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "-03", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 0 },
			end_date: TransitionDate { day: MonthWeekDay(3, 4, 0), time: 0 },
		}),
	},
};
const AMERICA_BAHIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767216356, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-191365200, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600, 592974000, 602042400, 624423600, 634701600, 656478000, 666756000, 687927600,
			697600800, 719982000, 728445600, 750826800, 761709600, 782276400, 793159200, 813726000,
			824004000, 844570800, 856058400, 876106800, 888717600, 908074800, 919562400, 938919600,
			951616800, 970974000, 982461600, 1003028400, 1013911200, 1036292400, 1045360800,
			1318734000, 1330221600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -9244, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_BAHIA_BANDERAS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			-873828000, -661539600, 28800, 828867600, 846403200, 860317200, 877852800, 891766800,
			909302400, 923216400, 941356800, 954666000, 972806400, 989139600, 1001836800,
			1018170000, 1035705600, 1049619600, 1067155200, 1081069200, 1099209600, 1112518800,
			1130659200, 1143968400, 1162108800, 1175418000, 1193558400, 1207472400, 1225008000,
			1238922000, 1256457600, 1270371600, 1288508400, 1301817600, 1319958000, 1333267200,
			1351407600, 1365321600, 1382857200, 1396771200, 1414306800, 1428220800, 1445756400,
			1459670400, 1477810800, 1491120000, 1509260400, 1522569600, 1540710000, 1554624000,
			1572159600, 1586073600, 1603609200, 1617523200, 1635663600, 1648972800, 1667113200,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 2, 1, 4, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 1, 3, 1, 3, 1, 3, 1, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5,
			2, 5, 2, 5, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25260, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "PST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_BARBADOS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1841256091, -874263600, -862682400, -841604400, -830714400, -811882800, -798660000,
			234943200, 244616400, 261554400, 276066000, 293004000, 307515600, 325058400, 338706000,
		],
		transition_types: &[2, 1, 2, 1, 2, 3, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14309, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -12600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "ADT", "AST", "-0330"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_BELEM: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767213964, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-191365200, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -11636, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_BELIZE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1822500432, -1616954400, -1606069800, -1585504800, -1574015400, -1554055200,
			-1542565800, -1522605600, -1511116200, -1490551200, -1479666600, -1459101600,
			-1448217000, -1427652000, -1416162600, -1396202400, -1384713000, -1364752800,
			-1353263400, -1333303200, -1321813800, -1301248800, -1290364200, -1269799200,
			-1258914600, -1238349600, -1226860200, -1206900000, -1195410600, -1175450400,
			-1163961000, -1143396000, -1132511400, -1111946400, -1101061800, -1080496800,
			-1069612200, -1049047200, -1037557800, -1017597600, -1006108200, -986148000,
			-974658600, -954093600, -943209000, -922644000, -911759400, -891194400, -879705000,
			-868212000, -769395600, -758746800, -701892000, -690402600, -670442400, -658953000,
			-638992800, -627503400, -606938400, -596053800, -575488800, -564604200, -544039200,
			-532549800, -512589600, -501100200, -481140000, -469650600, -449690400, -438201000,
			-417636000, -406751400, -386186400, -375301800, -354736800, -343247400, -323287200,
			-311797800, -291837600, -280348200, -259783200, -248898600, -228333600, -217449000,
			-196884000, -185999400, -165434400, -153945000, -133984800, -122495400, -102535200,
			-91045800, -70480800, -59596200, 123919200, 129618000, 409039200, 413874000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 5, 2, 5, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21168, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -19800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 14 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 18 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 22 },
		],
		time_zone_designations: &["LMT", "-0530", "CST", "CWT", "CPT", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_BOA_VISTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767211040, -1206954000, -1191358800, -1175371200, -1159822800, -633816000,
			-622065600, -602280000, -591829200, -570744000, -560206800, -539121600, -531349200,
			-191361600, -184194000, -155160000, -150066000, -128894400, -121122000, -99950400,
			-89586000, -68414400, -57963600, 499752000, 511239600, 530596800, 540270000, 562132800,
			571201200, 938923200, 951620400, 970977600, 971578800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14560, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-03", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_BOGOTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2707671824, -1739041424, 704869200, 729057600],
		transition_types: &[1, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -17776, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -17776, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "BMT", "-04", "-05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-05", offset: 18000 },
		dst_info: None,
	},
};
const AMERICA_BOISE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717640000, -1633269600, -1615129200, -1601820000, -1583679600, -1471788000,
			-880210800, -769395600, -765388800, -84380400, -68659200, -52930800, -37209600,
			-21481200, -5760000, 9968400, 25689600, 41418000, 57744000, 73472400, 89193600,
			104922000, 120643200, 129114000, 152092800, 162378000, 183542400, 199270800, 215596800,
			230720400, 247046400, 262774800, 278496000, 294224400, 309945600, 325674000, 341395200,
			357123600, 372844800, 388573200, 404899200, 420022800, 436348800, 452077200, 467798400,
			483526800, 499248000, 514976400, 530697600, 544611600, 562147200, 576061200, 594201600,
			607510800, 625651200, 638960400, 657100800, 671014800, 688550400, 702464400, 720000000,
			733914000, 752054400, 765363600, 783504000, 796813200, 814953600, 828867600, 846403200,
			860317200, 877852800, 891766800, 909302400, 923216400, 941356800, 954666000, 972806400,
			986115600, 1004256000, 1018170000, 1035705600, 1049619600, 1067155200, 1081069200,
			1099209600, 1112518800, 1130659200, 1143968400, 1162108800, 1173603600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 5, 3, 4, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			6, 5, 6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -27889, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "PDT", "PST", "MWT", "MPT", "MST", "MDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_BUENOS_AIRES: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372097972, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687927600, 699415200, 719377200, 731469600,
			938919600, 952052400, 1198983600, 1205632800, 1224385200, 1237082400,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 3, 5, 4,
			5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14028, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_CAMBRIDGE_BAY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1577923200, -880210800, -769395600, -765388800, 73472400, 89193600, 104922000,
			120643200, 136371600, 152092800, 167821200, 183542400, 199270800, 215596800, 230720400,
			247046400, 262774800, 278496000, 294224400, 309945600, 325674000, 341395200, 357123600,
			372844800, 388573200, 404899200, 420022800, 436348800, 452077200, 467798400, 483526800,
			499248000, 514976400, 530697600, 544611600, 562147200, 576061200, 594201600, 607510800,
			625651200, 638960400, 657100800, 671014800, 688550400, 702464400, 720000000, 733914000,
			752054400, 765363600, 783504000, 796813200, 814953600, 828867600, 846403200, 860317200,
			877852800, 891766800, 909302400, 923216400, 941356800, 954662400, 972802800, 973400400,
			986115600, 1004256000, 1018170000, 1035705600, 1049619600, 1067155200, 1081069200,
			1099209600, 1112518800, 1130659200, 1143968400, 1162108800, 1173603600,
		],
		transition_types: &[
			3, 1, 2, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 6, 5, 7, 6, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 28 },
		],
		time_zone_designations: &["-00", "MWT", "MPT", "MST", "MDT", "CDT", "CST", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_CAMPO_GRANDE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767212492, -1206954000, -1191358800, -1175371200, -1159822800, -633816000,
			-622065600, -602280000, -591829200, -570744000, -560206800, -539121600, -531349200,
			-191361600, -184194000, -155160000, -150066000, -128894400, -121122000, -99950400,
			-89586000, -68414400, -57963600, 499752000, 511239600, 530596800, 540270000, 562132800,
			571201200, 592977600, 602046000, 624427200, 634705200, 656481600, 666759600, 687931200,
			697604400, 719985600, 728449200, 750830400, 761713200, 782280000, 793162800, 813729600,
			824007600, 844574400, 856062000, 876110400, 888721200, 908078400, 919566000, 938923200,
			951620400, 970977600, 982465200, 1003032000, 1013914800, 1036296000, 1045364400,
			1066536000, 1076814000, 1099368000, 1108868400, 1129435200, 1140318000, 1162699200,
			1172372400, 1192334400, 1203217200, 1224388800, 1234666800, 1255838400, 1266721200,
			1287288000, 1298170800, 1318737600, 1330225200, 1350792000, 1361070000, 1382241600,
			1392519600, 1413691200, 1424574000, 1445140800, 1456023600, 1476590400, 1487473200,
			1508040000, 1518922800, 1541304000, 1550372400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13108, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-03", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_CANCUN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514743200, 377935200, 828860400, 846396000, 860310000, 877845600, 891759600,
			902037600, 909298800, 923212800, 941353200, 954662400, 972802800, 989136000,
			1001833200, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600, 1099206000,
			1112515200, 1130655600, 1143964800, 1162105200, 1175414400, 1193554800, 1207468800,
			1225004400, 1238918400, 1256454000, 1270368000, 1288508400, 1301817600, 1319958000,
			1333267200, 1351407600, 1365321600, 1382857200, 1396771200, 1414306800, 1422777600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20824, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "CST", "EDT", "EST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: None,
	},
};
const AMERICA_CARACAS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524505536, -1826739140, -157750200, 1197183600, 1462086000],
		transition_types: &[1, 2, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16064, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -16060, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -16200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "CMT", "-0430", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_CATAMARCA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372096212, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687931200, 699415200, 719377200, 731469600,
			938919600, 952052400, 1086058800, 1087704000, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 2, 4, 5, 4, 5, 3, 5, 2,
			5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15788, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_CAYENNE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1846269040, -71092800],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -12560, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-04", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_CHIHUAHUA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			828864000, 846399600, 860313600, 877849200, 891766800, 909302400, 923216400, 941356800,
			954666000, 972806400, 989139600, 1001836800, 1018170000, 1035705600, 1049619600,
			1067155200, 1081069200, 1099209600, 1112518800, 1130659200, 1143968400, 1162108800,
			1175418000, 1193558400, 1207472400, 1225008000, 1238922000, 1256457600, 1270371600,
			1288512000, 1301821200, 1319961600, 1333270800, 1351411200, 1365325200, 1382860800,
			1396774800, 1414310400, 1428224400, 1445760000, 1459674000, 1477814400, 1491123600,
			1509264000, 1522573200, 1540713600, 1554627600, 1572163200, 1586077200, 1603612800,
			1617526800, 1635667200, 1648976400, 1667116800,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 2, 4, 2, 4, 2, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3,
			1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25460, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_CIUDAD_JUAREZ: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			828864000, 846399600, 860313600, 877849200, 891766800, 909302400, 923216400, 941356800,
			954666000, 972806400, 989139600, 1001836800, 1018170000, 1035705600, 1049619600,
			1067155200, 1081069200, 1099209600, 1112518800, 1130659200, 1143968400, 1162108800,
			1175418000, 1193558400, 1207472400, 1225008000, 1238922000, 1256457600, 1268557200,
			1289116800, 1300006800, 1320566400, 1331456400, 1352016000, 1362906000, 1383465600,
			1394355600, 1414915200, 1425805200, 1446364800, 1457859600, 1478419200, 1489309200,
			1509868800, 1520758800, 1541318400, 1552208400, 1572768000, 1583658000, 1604217600,
			1615712400, 1636272000, 1647162000, 1667116800, 1669788000,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 2, 4, 2, 4, 2, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3,
			1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25556, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_CORAL_HARBOUR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524502512, -1946918424],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -19088, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -19176, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "CMT", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: None,
	},
};
const AMERICA_COSTA_RICA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524501427, -1545071027, 288770400, 297234000, 320220000, 328683600, 664264800,
			678344400, 695714400, 700635600,
		],
		transition_types: &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20173, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -20173, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 9 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "SJMT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_CUIABA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767212140, -1206954000, -1191358800, -1175371200, -1159822800, -633816000,
			-622065600, -602280000, -591829200, -570744000, -560206800, -539121600, -531349200,
			-191361600, -184194000, -155160000, -150066000, -128894400, -121122000, -99950400,
			-89586000, -68414400, -57963600, 499752000, 511239600, 530596800, 540270000, 562132800,
			571201200, 592977600, 602046000, 624427200, 634705200, 656481600, 666759600, 687931200,
			697604400, 719985600, 728449200, 750830400, 761713200, 782280000, 793162800, 813729600,
			824007600, 844574400, 856062000, 876110400, 888721200, 908078400, 919566000, 938923200,
			951620400, 970977600, 982465200, 1003032000, 1013914800, 1036296000, 1045364400,
			1099368000, 1108868400, 1129435200, 1140318000, 1162699200, 1172372400, 1192334400,
			1203217200, 1224388800, 1234666800, 1255838400, 1266721200, 1287288000, 1298170800,
			1318737600, 1330225200, 1350792000, 1361070000, 1382241600, 1392519600, 1413691200,
			1424574000, 1445140800, 1456023600, 1476590400, 1487473200, 1508040000, 1518922800,
			1541304000, 1550372400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13460, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-03", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_DANMARKSHAVN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1686091520, 323845200, 338950800, 354675600, 370400400, 386125200, 401850000,
			417574800, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400, 528253200,
			543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400,
			670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800,
			796179600, 811904400, 820465200,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -4480, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "-03", "-02", "GMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "GMT", offset: 0 }, dst_info: None },
};
const AMERICA_DAWSON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2188996940, -1632056400, -1615125600, -1596978000, -1583164800, -880203600,
			-769395600, -765381600, -147884400, -131554800, 120646800, 325677600, 341398800,
			357127200, 372848400, 388576800, 404902800, 420026400, 436352400, 452080800, 467802000,
			483530400, 499251600, 514980000, 530701200, 544615200, 562150800, 576064800, 594205200,
			607514400, 625654800, 638964000, 657104400, 671018400, 688554000, 702468000, 720003600,
			733917600, 752058000, 765367200, 783507600, 796816800, 814957200, 828871200, 846406800,
			860320800, 877856400, 891770400, 909306000, 923220000, 941360400, 954669600, 972810000,
			986119200, 1004259600, 1018173600, 1035709200, 1049623200, 1067158800, 1081072800,
			1099213200, 1112522400, 1130662800, 1143972000, 1162112400, 1173607200, 1194166800,
			1205056800, 1225616400, 1236506400, 1257066000, 1268560800, 1289120400, 1300010400,
			1320570000, 1331460000, 1352019600, 1362909600, 1383469200, 1394359200, 1414918800,
			1425808800, 1446368400, 1457863200, 1478422800, 1489312800, 1509872400, 1520762400,
			1541322000, 1552212000, 1572771600, 1583661600, 1604214000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 5, 2, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6,
			7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7,
			6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6,
			7, 6, 7, 6, 7, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -33460, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 25 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 33 },
		],
		time_zone_designations: &["LMT", "YDT", "YST", "YWT", "YPT", "YDDT", "PST", "PDT", "MST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const AMERICA_DAWSON_CREEK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713881544, -1632060000, -1615129200, -880207200, -769395600, -765385200, -715788000,
			-702486000, -684338400, -671036400, -652888800, -639586800, -620834400, -608137200,
			-589384800, -576082800, -557935200, -544633200, -526485600, -513183600, -495036000,
			-481734000, -463586400, -450284400, -431532000, -418230000, -400082400, -386780400,
			-368632800, -355330800, -337183200, -323881200, -305733600, -292431600, -273679200,
			-260982000, -242229600, -226508400, -210780000, -195058800, -179330400, -163609200,
			-147880800, -131554800, -116431200, -100105200, -84376800, -68655600, -52927200,
			-37206000, -21477600, -5756400, 9972000, 25693200, 41421600, 57747600, 73476000,
			84013200,
		],
		transition_types: &[
			2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -28856, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "PDT", "PST", "PWT", "PPT", "MST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const AMERICA_EIRUNEPE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767208832, -1206950400, -1191355200, -1175367600, -1159819200, -633812400,
			-622062000, -602276400, -591825600, -570740400, -560203200, -539118000, -531345600,
			-191358000, -184190400, -155156400, -150062400, -128890800, -121118400, -99946800,
			-89582400, -68410800, -57960000, 499755600, 511243200, 530600400, 540273600, 562136400,
			571204800, 750834000, 761716800, 1214283600, 1384056000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16768, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-04", "-05", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-05", offset: 18000 },
		dst_info: None,
	},
};
const AMERICA_EL_SALVADOR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1546279392, 547020000, 559717200, 578469600, 591166800],
		transition_types: &[2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21408, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_FORT_NELSON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713880953, -1632060000, -1615129200, -880207200, -769395600, -765385200, -715788000,
			-702486000, -684338400, -671036400, -652888800, -639586800, -620834400, -608137200,
			-589384800, -576082800, -557935200, -544633200, -526485600, -513183600, -495036000,
			-481734000, -463586400, -450284400, -431532000, -418230000, -400082400, -386780400,
			-368632800, -355330800, -337183200, -323881200, -305733600, -292431600, -273679200,
			-260982000, -242229600, -226508400, -210780000, -195058800, -179330400, -163609200,
			-147880800, -131554800, -116431200, -100105200, -84376800, -68655600, -52927200,
			-37206000, -21477600, -5756400, 9972000, 25693200, 41421600, 57747600, 73476000,
			89197200, 104925600, 120646800, 136375200, 152096400, 167824800, 183546000, 199274400,
			215600400, 230724000, 247050000, 262778400, 278499600, 294228000, 309949200, 325677600,
			341398800, 357127200, 372848400, 388576800, 404902800, 420026400, 436352400, 452080800,
			467802000, 483530400, 499251600, 514980000, 530701200, 544615200, 562150800, 576064800,
			594205200, 607514400, 625654800, 638964000, 657104400, 671018400, 688554000, 702468000,
			720003600, 733917600, 752058000, 765367200, 783507600, 796816800, 814957200, 828871200,
			846406800, 860320800, 877856400, 891770400, 909306000, 923220000, 941360400, 954669600,
			972810000, 986119200, 1004259600, 1018173600, 1035709200, 1049623200, 1067158800,
			1081072800, 1099213200, 1112522400, 1130662800, 1143972000, 1162112400, 1173607200,
			1194166800, 1205056800, 1225616400, 1236506400, 1257066000, 1268560800, 1289120400,
			1300010400, 1320570000, 1331460000, 1352019600, 1362909600, 1383469200, 1394359200,
			1414918800, 1425808800,
		],
		transition_types: &[
			2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -29447, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "PDT", "PST", "PWT", "PPT", "MST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const AMERICA_FORTALEZA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767216360, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-191365200, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600, 592974000, 602042400, 624423600, 634701600, 938919600, 951616800, 970974000,
			972180000, 1003028400, 1013911200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -9240, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_GLACE_BAY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2131646412, -1632074400, -1615143600, -880221600, -769395600, -765399600, -526500000,
			-513198000, 73461600, 89182800, 104911200, 120632400, 136360800, 152082000, 167810400,
			183531600, 199260000, 215586000, 230709600, 247035600, 262764000, 278485200, 294213600,
			309934800, 325663200, 341384400, 357112800, 372834000, 388562400, 404888400, 420012000,
			436338000, 452066400, 467787600, 483516000, 499237200, 514965600, 530686800, 544600800,
			562136400, 576050400, 594190800, 607500000, 625640400, 638949600, 657090000, 671004000,
			688539600, 702453600, 719989200, 733903200, 752043600, 765352800, 783493200, 796802400,
			814942800, 828856800, 846392400, 860306400, 877842000, 891756000, 909291600, 923205600,
			941346000, 954655200, 972795600, 986104800, 1004245200, 1018159200, 1035694800,
			1049608800, 1067144400, 1081058400, 1099198800, 1112508000, 1130648400, 1143957600,
			1162098000, 1173592800,
		],
		transition_types: &[
			2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14388, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "ADT", "AST", "AWT", "APT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ADT", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_GODTHAB: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1686083584, 323845200, 338950800, 354675600, 370400400, 386125200, 401850000,
			417574800, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400, 528253200,
			543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400,
			670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800,
			796179600, 811904400, 828234000, 846378000, 859683600, 877827600, 891133200, 909277200,
			922582800, 941331600, 954032400, 972781200, 985482000, 1004230800, 1017536400,
			1035680400, 1048986000, 1067130000, 1080435600, 1099184400, 1111885200, 1130634000,
			1143334800, 1162083600, 1174784400, 1193533200, 1206838800, 1224982800, 1238288400,
			1256432400, 1269738000, 1288486800, 1301187600, 1319936400, 1332637200, 1351386000,
			1364691600, 1382835600, 1396141200, 1414285200, 1427590800, 1445734800, 1459040400,
			1477789200, 1490490000, 1509238800, 1521939600, 1540688400, 1553994000, 1572138000,
			1585443600, 1603587600, 1616893200, 1635642000, 1648342800, 1667091600, 1679792400,
			1698541200,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -12416, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-03", "-02", "-02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-02", offset: 7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "-01", offset: 3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: -3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 0 },
		}),
	},
};
const AMERICA_GOOSE_BAY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713895900, -1632076148, -1615145348, -1096921748, -1061670600, -1048973400,
			-1030221000, -1017523800, -998771400, -986074200, -966717000, -954624600, -935267400,
			-922570200, -903817800, -891120600, -872368200, -769395600, -765401400, -746044200,
			-733347000, -714594600, -701897400, -683145000, -670447800, -651695400, -638998200,
			-619641000, -606943800, -589401000, -576099000, -557951400, -544649400, -526501800,
			-513199800, -495052200, -481750200, -463602600, -450300600, -431548200, -418246200,
			-400098600, -386796600, -368649000, -355347000, -337199400, -323897400, -305749800,
			-289423800, -273695400, -257974200, -242245800, -226524600, -210796200, -195075000,
			-179346600, -163625400, -147897000, -131571000, -119903400, -116445600, -100119600,
			-84391200, -68670000, -52941600, -37220400, -21492000, -5770800, 9957600, 25678800,
			41407200, 57733200, 73461600, 89182800, 104911200, 120632400, 136360800, 152082000,
			167810400, 183531600, 199260000, 215586000, 230709600, 247035600, 262764000, 278485200,
			294213600, 309934800, 325663200, 341384400, 357112800, 372834000, 388562400, 404888400,
			420012000, 436338000, 452066400, 467787600, 483516000, 499237200, 514965600, 530686800,
			544593660, 562129260, 576043260, 594180060, 607492860, 625633260, 638942460, 657082860,
			670996860, 688532460, 702446460, 719982060, 733896060, 752036460, 765345660, 783486060,
			796795260, 814935660, 828849660, 846385260, 860299260, 877834860, 891748860, 909284460,
			923198460, 941338860, 954648060, 972788460, 986097660, 1004238060, 1018152060,
			1035687660, 1049601660, 1067137260, 1081051260, 1099191660, 1112500860, 1130641260,
			1143950460, 1162090860, 1173585660, 1194145260, 1205035260, 1225594860, 1236484860,
			1257044460, 1268539260, 1289098860, 1299988860, 1320555600,
		],
		transition_types: &[
			1, 2, 1, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 6, 5, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7,
			8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 9, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8,
			7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7,
			8, 7, 8, 7, 8, 7, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14500, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -12652, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -9052, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -12600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 28 },
		],
		time_zone_designations: &[
			"LMT", "NST", "NDT", "NST", "NDT", "NPT", "NWT", "ADT", "AST", "ADDT",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ADT", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_GRAND_TURK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524504528, -1827687170, 294217200, 309938400, 325666800, 341388000, 357116400,
			372837600, 388566000, 404892000, 420015600, 436341600, 452070000, 467791200, 483519600,
			499240800, 514969200, 530690400, 544604400, 562140000, 576054000, 594194400, 607503600,
			625644000, 638953200, 657093600, 671007600, 688543200, 702457200, 719992800, 733906800,
			752047200, 765356400, 783496800, 796806000, 814946400, 828860400, 846396000, 860310000,
			877845600, 891759600, 909295200, 923209200, 941349600, 954658800, 972799200, 986108400,
			1004248800, 1018162800, 1035698400, 1049612400, 1067148000, 1081062000, 1099202400,
			1112511600, 1130652000, 1143961200, 1162101600, 1173596400, 1194156000, 1205046000,
			1225605600, 1236495600, 1257055200, 1268550000, 1289109600, 1299999600, 1320559200,
			1331449200, 1352008800, 1362898800, 1383458400, 1394348400, 1414908000, 1425798000,
			1520751600,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -17072, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18430, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "KMT", "EST", "EDT", "AST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_GUATEMALA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1617040676, 123055200, 130914000, 422344800, 433054800, 669708000, 684219600,
			1146376800, 1159678800,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21724, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_GUAYAQUIL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524502440, -1230749160, 722926800, 728884800],
		transition_types: &[1, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -19160, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18840, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "QMT", "-04", "-05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-05", offset: 18000 },
		dst_info: None,
	},
};
const AMERICA_GUYANA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1843589241, -1730577600, 176096700, 701841600],
		transition_types: &[1, 2, 3, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13959, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -13500, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "-04", "-0345", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_HERMOSILLO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			-873828000, -661539600, 28800, 828867600, 846403200, 860317200, 877852800, 891766800,
			909302400,
		],
		transition_types: &[1, 2, 1, 3, 1, 2, 1, 4, 1, 3, 1, 3, 1, 3, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -26632, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "PST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const AMERICA_INDIANA_MARENGO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -589392000, -576090000, -495043200, -481741200, -463593600,
			-450291600, -431539200, -418237200, -400089600, -386787600, -368640000, -355338000,
			-337190400, -323888400, -305740800, -292438800, -273686400, -21488400, -5767200,
			9961200, 25682400, 41410800, 57736800, 73465200, 89186400, 104914800, 120636000,
			126687600, 152089200, 162370800, 183535200, 1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5, 6, 5, 6, 5,
			6, 5, 6, 5, 6, 5, 1, 5, 6, 5, 6, 5, 6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20723, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_INDIANA_PETERSBURG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -462996000, -450291600, -431539200, -418237200, -400089600,
			-386787600, -368640000, -355338000, -337190400, -323888400, -305740800, -292438800,
			-273686400, -257965200, -242236800, -226515600, -210787200, -195066000, -179337600,
			-163616400, -147888000, -100112400, -84384000, -68662800, -52934400, -37213200,
			-21484800, -5763600, 9964800, 25686000, 41414400, 57740400, 73468800, 89190000,
			104918400, 120639600, 126691200, 152089200, 162374400, 183538800, 199267200, 215593200,
			230716800, 247042800, 1143961200, 1162105200, 1173600000, 1194159600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 5, 1, 2, 1, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20947, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_INDIANA_TELL_CITY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -462996000, -450291600, -431539200, -418237200, -400089600,
			-386787600, -368640000, -355338000, -337190400, -323888400, -305740800, -292438800,
			-273686400, -257965200, -242236800, -226515600, -210787200, -195066000, -179337600,
			-68662800, -52934400, -37213200, -21484800, -5767200, 9961200, 25682400, 1143961200,
			1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5, 2, 1,
			2, 6, 5, 6, 5, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20823, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_INDIANA_VEVAY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -495043200, -21488400, -5767200, 9961200, 25682400, 41410800,
			57736800, 73465200, 89186400, 1143961200, 1162101600, 1173596400,
		],
		transition_types: &[2, 1, 2, 1, 2, 3, 4, 2, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20416, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_INDIANA_VINCENNES: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -747244800, -733942800, -526492800, -513190800, -495043200,
			-481741200, -462996000, -450291600, -431539200, -418237200, -400089600, -386787600,
			-368640000, -355338000, -337190400, -323888400, -305740800, -289414800, -273686400,
			-260989200, -242236800, -226515600, -210787200, -195066000, -179337600, -21488400,
			-5767200, 9961200, 25682400, 1143961200, 1162105200, 1173600000, 1194159600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 5, 6, 5, 6, 5, 1, 2, 1, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21007, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_INDIANA_WINAMAC: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -747244800, -733942800, -715795200, -702493200, -684345600,
			-671043600, -652896000, -639594000, -620841600, -608144400, -589392000, -576090000,
			-557942400, -544640400, -526492800, -513190800, -495043200, -481741200, -463593600,
			-447267600, -431539200, -415818000, -400089600, -386787600, -368640000, -355338000,
			-337190400, -323888400, -305740800, -292438800, -273686400, -21488400, -5767200,
			9961200, 25682400, 1143961200, 1162105200, 1173600000, 1194156000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 5, 6, 5, 6, 5, 1, 2, 6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20785, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_INUVIK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-536457600, 73476000, 89197200, 104925600, 120646800, 136375200, 152096400, 167824800,
			183546000, 199274400, 215600400, 230724000, 247050000, 262778400, 278499600, 294228000,
			309945600, 325674000, 341395200, 357123600, 372844800, 388573200, 404899200, 420022800,
			436348800, 452077200, 467798400, 483526800, 499248000, 514976400, 530697600, 544611600,
			562147200, 576061200, 594201600, 607510800, 625651200, 638960400, 657100800, 671014800,
			688550400, 702464400, 720000000, 733914000, 752054400, 765363600, 783504000, 796813200,
			814953600, 828867600, 846403200, 860317200, 877852800, 891766800, 909302400, 923216400,
			941356800, 954666000, 972806400, 986115600, 1004256000, 1018170000, 1035705600,
			1049619600, 1067155200, 1081069200, 1099209600, 1112518800, 1130659200, 1143968400,
			1162108800, 1173603600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["-00", "PDT", "PST", "MST", "MDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_JUJUY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372096328, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 657086400, 669178800, 686721600, 699415200, 719377200, 731469600,
			938919600, 952052400, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 2, 3, 2, 4, 5, 4, 5, 3, 5, 4,
			5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15672, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_JUNEAU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188954939, -880207200, -769395600, -765385200, -21477600, -5756400,
			9972000, 25693200, 41421600, 57747600, 73476000, 89197200, 104925600, 120646800,
			126698400, 152096400, 162381600, 183546000, 199274400, 215600400, 230724000, 247050000,
			262778400, 278499600, 294228000, 309949200, 325677600, 341402400, 357127200, 372848400,
			388576800, 404902800, 420026400, 436352400, 439030800, 452084400, 467805600, 483534000,
			499255200, 514983600, 530704800, 544618800, 562154400, 576068400, 594208800, 607518000,
			625658400, 638967600, 657108000, 671022000, 688557600, 702471600, 720007200, 733921200,
			752061600, 765370800, 783511200, 796820400, 814960800, 828874800, 846410400, 860324400,
			877860000, 891774000, 909309600, 923223600, 941364000, 954673200, 972813600, 986122800,
			1004263200, 1018177200, 1035712800, 1049626800, 1067162400, 1081076400, 1099216800,
			1112526000, 1130666400, 1143975600, 1162116000, 1173610800,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 6, 2,
			5, 2, 5, 2, 5, 7, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9,
			8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 54139, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -32261, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 28 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 33 },
		],
		time_zone_designations: &[
			"LMT", "LMT", "PST", "PWT", "PPT", "PDT", "YDT", "YST", "AKDT", "AKST",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AKST", offset: 32400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AKDT", offset: 28800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_KENTUCKY_MONTICELLO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -52934400, -37213200, -21484800, -5763600, 9964800, 25686000,
			41414400, 57740400, 73468800, 89190000, 104918400, 120639600, 126691200, 152089200,
			162374400, 183538800, 199267200, 215593200, 230716800, 247042800, 262771200, 278492400,
			294220800, 309942000, 325670400, 341391600, 357120000, 372841200, 388569600, 404895600,
			420019200, 436345200, 452073600, 467794800, 483523200, 499244400, 514972800, 530694000,
			544608000, 562143600, 576057600, 594198000, 607507200, 625647600, 638956800, 657097200,
			671011200, 688546800, 702460800, 719996400, 733910400, 752050800, 765360000, 783500400,
			796809600, 814950000, 828864000, 846399600, 860313600, 877849200, 891763200, 909298800,
			923212800, 941353200, 954662400, 972802800, 986108400, 1004248800, 1018162800,
			1035698400, 1049612400, 1067148000, 1081062000, 1099202400, 1112511600, 1130652000,
			1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20364, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EDT", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_LA_PAZ: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524505244, -1205954844, -1192307244],
		transition_types: &[1, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16356, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -16356, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -12756, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "BST", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_LIMA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524503108, -1938538284, -1009825200, -1002052800, -986756400, -971035200, -955306800,
			-939585600, 504939600, 512712000, 536475600, 544248000, 631170000, 638942400,
			757400400, 765172800,
		],
		transition_types: &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -18492, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18516, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "LMT", "-04", "-05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-05", offset: 18000 },
		dst_info: None,
	},
};
const AMERICA_LOUISVILLE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -1535904000,
			-1525280400, -905097600, -891795600, -880214400, -769395600, -765392400, -747251940,
			-744224400, -620841600, -608144400, -589392000, -576090000, -557942400, -544640400,
			-526492800, -513190800, -495043200, -481741200, -463593600, -450291600, -431539200,
			-415818000, -400089600, -384368400, -368640000, -352918800, -337190400, -321469200,
			-305740800, -289414800, -273686400, -266432400, -52938000, -37216800, -21488400,
			-5767200, 9961200, 25682400, 41410800, 57736800, 73465200, 89186400, 104914800,
			120636000, 126687600, 152089200, 162370800, 183535200, 199263600, 215589600, 230713200,
			247039200, 262767600, 278488800, 294217200, 309938400, 325666800, 341388000, 357116400,
			372837600, 388566000, 404892000, 420015600, 436341600, 452070000, 467791200, 483519600,
			499240800, 514969200, 530690400, 544604400, 562140000, 576054000, 594194400, 607503600,
			625644000, 638953200, 657093600, 671007600, 688543200, 702457200, 719992800, 733906800,
			752047200, 765356400, 783496800, 796806000, 814946400, 828860400, 846396000, 860310000,
			877845600, 891759600, 909295200, 923209200, 941349600, 954658800, 972799200, 986108400,
			1004248800, 1018162800, 1035698400, 1049612400, 1067148000, 1081062000, 1099202400,
			1112511600, 1130652000, 1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 1, 5, 6, 5, 6, 5, 6, 5,
			6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20582, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_MACEIO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767217028, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-191365200, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600, 592974000, 602042400, 624423600, 634701600, 813726000, 824004000, 938919600,
			951616800, 970974000, 972180000, 1003028400, 1013911200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -8572, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_MANAGUA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524500892, -1121105688, 105084000, 161758800, 290584800, 299134800, 322034400,
			330584400, 694260000, 717310800, 725868000, 852094800, 1113112800, 1128229200,
			1146384000, 1159682400,
		],
		transition_types: &[1, 2, 3, 2, 4, 2, 4, 2, 3, 2, 3, 2, 4, 2, 4, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20708, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -20712, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "MMT", "CST", "EST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_MARTINIQUE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524506940, -1851537340, 323841600, 338958000],
		transition_types: &[1, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14660, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14660, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 13 },
		],
		time_zone_designations: &["LMT", "FFMT", "AST", "ADT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_MATAMOROS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514743200, 576057600, 594198000, 828864000, 846399600, 860313600, 877849200,
			891763200, 909298800, 923212800, 941353200, 954662400, 972802800, 989136000,
			1001833200, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600, 1099206000,
			1112515200, 1130655600, 1143964800, 1162105200, 1175414400, 1193554800, 1207468800,
			1225004400, 1238918400, 1256454000, 1268553600,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -23400, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "CST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_MENDOZA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372095484, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 655963200, 667796400, 687499200, 699418800, 719380800, 731469600,
			938919600, 952052400, 1085281200, 1096171200, 1198983600, 1205632800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 2, 3, 2, 3, 2, 4, 5, 3, 5, 2,
			5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16516, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_MENOMINEE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2659759773, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -747244800, -733942800, -116438400, -100112400, -21484800,
			104914800, 120639600, 126691200, 152089200, 162374400, 183538800, 199267200, 215593200,
			230716800, 247042800, 262771200, 278492400, 294220800, 309942000, 325670400, 341391600,
			357120000, 372841200, 388569600, 404895600, 420019200, 436345200, 452073600, 467794800,
			483523200, 499244400, 514972800, 530694000, 544608000, 562143600, 576057600, 594198000,
			607507200, 625647600, 638956800, 657097200, 671011200, 688546800, 702460800, 719996400,
			733910400, 752050800, 765360000, 783500400, 796809600, 814950000, 828864000, 846399600,
			860313600, 877849200, 891763200, 909298800, 923212800, 941353200, 954662400, 972802800,
			986112000, 1004252400, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600,
			1099206000, 1112515200, 1130655600, 1143964800, 1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 5, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21027, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_MERIDA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514743200, 377935200, 407653200, 828864000, 846399600, 860313600, 877849200,
			891763200, 909298800, 923212800, 941353200, 954662400, 972802800, 989136000,
			1001833200, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600, 1099206000,
			1112515200, 1130655600, 1143964800, 1162105200, 1175414400, 1193554800, 1207468800,
			1225004400, 1238918400, 1256454000, 1270368000, 1288508400, 1301817600, 1319958000,
			1333267200, 1351407600, 1365321600, 1382857200, 1396771200, 1414306800, 1428220800,
			1445756400, 1459670400, 1477810800, 1491120000, 1509260400, 1522569600, 1540710000,
			1554624000, 1572159600, 1586073600, 1603609200, 1617523200, 1635663600, 1648972800,
			1667113200,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21508, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "CST", "EST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_METLAKATLA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188955622, -880207200, -769395600, -765385200, -21477600, -5756400,
			9972000, 25693200, 41421600, 57747600, 73476000, 89197200, 104925600, 120646800,
			126698400, 152096400, 162381600, 183546000, 199274400, 215600400, 230724000, 247050000,
			262778400, 278499600, 294228000, 309949200, 325677600, 341398800, 357127200, 372848400,
			388576800, 404902800, 420026400, 436352400, 1446372000, 1457866800, 1478426400,
			1489316400, 1509876000, 1520766000, 1541325600, 1547978400, 1552215600,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2,
			5, 2, 5, 2, 5, 2, 6, 7, 6, 7, 6, 7, 2, 6, 7,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 54822, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -31578, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 25 },
		],
		time_zone_designations: &["LMT", "LMT", "PST", "PWT", "PPT", "PDT", "AKST", "AKDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AKST", offset: 32400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AKDT", offset: 28800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_MIQUELON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1850328920, 326001600, 544597200, 562132800, 576046800, 594187200, 607496400,
			625636800, 638946000, 657086400, 671000400, 688536000, 702450000, 719985600, 733899600,
			752040000, 765349200, 783489600, 796798800, 814939200, 828853200, 846388800, 860302800,
			877838400, 891752400, 909288000, 923202000, 941342400, 954651600, 972792000, 986101200,
			1004241600, 1018155600, 1035691200, 1049605200, 1067140800, 1081054800, 1099195200,
			1112504400, 1130644800, 1143954000, 1162094400, 1173589200,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13480, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "AST", "-03", "-02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "-02", offset: 7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_MONCTON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2715882052, -2131642800, -1632074400, -1615143600, -1153681200, -1145822400,
			-1122231600, -1114372800, -1090782000, -1082923200, -1059332400, -1051473600,
			-1027882800, -1020024000, -996433200, -988574400, -965674800, -955396800, -934743600,
			-923947200, -904503600, -891892800, -880221600, -769395600, -765399600, -747252000,
			-733950000, -715802400, -702500400, -684352800, -671050800, -652903200, -639601200,
			-620848800, -608151600, -589399200, -576097200, -557949600, -544647600, -526500000,
			-513198000, -495050400, -481748400, -463600800, -450298800, -431546400, -418244400,
			-400096800, -384375600, -368647200, -352926000, -337197600, -321476400, -305748000,
			-289422000, -273693600, -257972400, -242244000, -226522800, -210794400, -195073200,
			-179344800, -163623600, -147895200, -131569200, -116445600, -100119600, -84391200,
			-68670000, -52941600, -37220400, -21492000, -5770800, 9957600, 25678800, 41407200,
			57733200, 73461600, 89182800, 136360800, 152082000, 167810400, 183531600, 199260000,
			215586000, 230709600, 247035600, 262764000, 278485200, 294213600, 309934800, 325663200,
			341384400, 357112800, 372834000, 388562400, 404888400, 420012000, 436338000, 452066400,
			467787600, 483516000, 499237200, 514965600, 530686800, 544600800, 562136400, 576050400,
			594190800, 607500000, 625640400, 638949600, 657090000, 671004000, 688539600, 702453600,
			719989200, 733896060, 752036460, 765345660, 783486060, 796795260, 814935660, 828849660,
			846385260, 860299260, 877834860, 891748860, 909284460, 923198460, 941338860, 954648060,
			972788460, 986097660, 1004238060, 1018152060, 1035687660, 1049601660, 1067137260,
			1081051260, 1099191660, 1112500860, 1130641260, 1143950460, 1162090860, 1173592800,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15548, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "EST", "ADT", "AST", "AWT", "APT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ADT", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_MONTERREY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514743200, 576057600, 594198000, 828864000, 846399600, 860313600, 877849200,
			891763200, 909298800, 923212800, 941353200, 954662400, 972802800, 989136000,
			1001833200, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600, 1099206000,
			1112515200, 1130655600, 1143964800, 1162105200, 1175414400, 1193554800, 1207468800,
			1225004400, 1238918400, 1256454000, 1270368000, 1288508400, 1301817600, 1319958000,
			1333267200, 1351407600, 1365321600, 1382857200, 1396771200, 1414306800, 1428220800,
			1445756400, 1459670400, 1477810800, 1491120000, 1509260400, 1522569600, 1540710000,
			1554624000, 1572159600, 1586073600, 1603609200, 1617523200, 1635663600, 1648972800,
			1667113200,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -24076, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "CST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_MONTEVIDEO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1942690509, -1567455309, -1459627200, -1443819600, -1428006600, -1412283600,
			-1396470600, -1380747600, -1141590600, -1128286800, -1110141000, -1096837200,
			-1078691400, -1065387600, -1047241800, -1033938000, -1015187400, -1002488400,
			-983737800, -971038800, -954707400, -938984400, -920838600, -907534800, -896819400,
			-853621200, -845847000, -334789200, -319671000, -314226000, -309996000, -149720400,
			-134604000, -50446800, -34205400, 9860400, 14176800, 72846000, 80100000, 127278000,
			132111000, 147234600, 156913200, 165376800, 219812400, 226461600, 250052400, 257911200,
			282711600, 289360800, 294202800, 322020000, 566449200, 573012000, 597812400, 605066400,
			625633200, 635911200, 656478000, 667965600, 688532400, 699415200, 719377200, 730864800,
			1095562800, 1111896000, 1128834000, 1142136000, 1159678800, 1173585600, 1191733200,
			1205035200, 1223182800, 1236484800, 1254632400, 1268539200, 1286082000, 1299988800,
			1317531600, 1331438400, 1349586000, 1362888000, 1381035600, 1394337600, 1412485200,
			1425787200,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 6, 5, 6, 5,
			7, 5, 7, 5, 6, 5, 7, 5, 7, 5, 8, 6, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5,
			7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13491, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -13491, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -12600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 18 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 22 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 28 },
			LocalTimeTypeRecord { utc_offset: -5400, is_dst: true, idx: 32 },
		],
		time_zone_designations: &[
			"LMT", "MMT", "-04", "-0330", "-03", "-03", "-0230", "-02", "-0130",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_NOME: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188947502, -880196400, -769395600, -765374400, -86878800, -21466800,
			-5745600, 9982800, 25704000, 41432400, 57758400, 73486800, 89208000, 104936400,
			120657600, 126709200, 152107200, 162392400, 183556800, 199285200, 215611200, 230734800,
			247060800, 262789200, 278510400, 294238800, 309960000, 325688400, 341409600, 357138000,
			372859200, 388587600, 404913600, 420037200, 436363200, 439030800, 452084400, 467805600,
			483534000, 499255200, 514983600, 530704800, 544618800, 562154400, 576068400, 594208800,
			607518000, 625658400, 638967600, 657108000, 671022000, 688557600, 702471600, 720007200,
			733921200, 752061600, 765370800, 783511200, 796820400, 814960800, 828874800, 846410400,
			860324400, 877860000, 891774000, 909309600, 923223600, 941364000, 954673200, 972813600,
			986122800, 1004263200, 1018177200, 1035712800, 1049626800, 1067162400, 1081076400,
			1099216800, 1112526000, 1130666400, 1143975600, 1162116000, 1173610800,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 7, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
			9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 46702, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -39698, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 28 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 33 },
		],
		time_zone_designations: &[
			"LMT", "LMT", "NST", "NWT", "NPT", "BST", "BDT", "YST", "AKDT", "AKST",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AKST", offset: 32400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AKDT", offset: 28800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_NORTH_DAKOTA_BEULAH: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717643600, -1633273200, -1615132800, -1601823600, -1583683200, -880210800,
			-769395600, -765388800, -84380400, -68659200, -52930800, -37209600, -21481200,
			-5760000, 9968400, 25689600, 41418000, 57744000, 73472400, 89193600, 104922000,
			120643200, 126694800, 152092800, 162378000, 183542400, 199270800, 215596800, 230720400,
			247046400, 262774800, 278496000, 294224400, 309945600, 325674000, 341395200, 357123600,
			372844800, 388573200, 404899200, 420022800, 436348800, 452077200, 467798400, 483526800,
			499248000, 514976400, 530697600, 544611600, 562147200, 576061200, 594201600, 607510800,
			625651200, 638960400, 657100800, 671014800, 688550400, 702464400, 720000000, 733914000,
			752054400, 765363600, 783504000, 796813200, 814953600, 828867600, 846403200, 860317200,
			877852800, 891766800, 909302400, 923216400, 941356800, 954666000, 972806400, 986115600,
			1004256000, 1018170000, 1035705600, 1049619600, 1067155200, 1081069200, 1099209600,
			1112518800, 1130659200, 1143968400, 1162108800, 1173603600, 1194163200, 1205053200,
			1225612800, 1236502800, 1257062400, 1268557200, 1289116800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -24427, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_NORTH_DAKOTA_CENTER: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717643600, -1633273200, -1615132800, -1601823600, -1583683200, -880210800,
			-769395600, -765388800, -84380400, -68659200, -52930800, -37209600, -21481200,
			-5760000, 9968400, 25689600, 41418000, 57744000, 73472400, 89193600, 104922000,
			120643200, 126694800, 152092800, 162378000, 183542400, 199270800, 215596800, 230720400,
			247046400, 262774800, 278496000, 294224400, 309945600, 325674000, 341395200, 357123600,
			372844800, 388573200, 404899200, 420022800, 436348800, 452077200, 467798400, 483526800,
			499248000, 514976400, 530697600, 544611600, 562147200, 576061200, 594201600, 607510800,
			625651200, 638960400, 657100800, 671014800, 688550400, 702464400, 720000000, 733910400,
			752050800, 765360000, 783500400, 796809600, 814950000, 828864000, 846399600, 860313600,
			877849200, 891763200, 909298800, 923212800, 941353200, 954662400, 972802800, 986112000,
			1004252400, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600, 1099206000,
			1112515200, 1130655600, 1143964800, 1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -24312, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 24 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_NORTH_DAKOTA_NEW_SALEM: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717643600, -1633273200, -1615132800, -1601823600, -1583683200, -880210800,
			-769395600, -765388800, -84380400, -68659200, -52930800, -37209600, -21481200,
			-5760000, 9968400, 25689600, 41418000, 57744000, 73472400, 89193600, 104922000,
			120643200, 126694800, 152092800, 162378000, 183542400, 199270800, 215596800, 230720400,
			247046400, 262774800, 278496000, 294224400, 309945600, 325674000, 341395200, 357123600,
			372844800, 388573200, 404899200, 420022800, 436348800, 452077200, 467798400, 483526800,
			499248000, 514976400, 530697600, 544611600, 562147200, 576061200, 594201600, 607510800,
			625651200, 638960400, 657100800, 671014800, 688550400, 702464400, 720000000, 733914000,
			752054400, 765363600, 783504000, 796813200, 814953600, 828867600, 846403200, 860317200,
			877852800, 891766800, 909302400, 923216400, 941356800, 954666000, 972806400, 986115600,
			1004256000, 1018170000, 1035705600, 1049619600, 1067155200, 1081065600, 1099206000,
			1112515200, 1130655600, 1143964800, 1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 6, 5, 6, 5, 6, 5,
			6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -24339, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 24 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_OJINAGA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			828864000, 846399600, 860313600, 877849200, 891766800, 909302400, 923216400, 941356800,
			954666000, 972806400, 989139600, 1001836800, 1018170000, 1035705600, 1049619600,
			1067155200, 1081069200, 1099209600, 1112518800, 1130659200, 1143968400, 1162108800,
			1175418000, 1193558400, 1207472400, 1225008000, 1238922000, 1256457600, 1268557200,
			1289116800, 1300006800, 1320566400, 1331456400, 1352016000, 1362906000, 1383465600,
			1394355600, 1414915200, 1425805200, 1446364800, 1457859600, 1478419200, 1489309200,
			1509868800, 1520758800, 1541318400, 1552208400, 1572768000, 1583658000, 1604217600,
			1615712400, 1636272000, 1647162000, 1667116800,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 2, 4, 2, 4, 2, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3,
			1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25060, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_PANGNIRTUNG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-865296000, -769395600, -765396000, 73465200, 89186400, 104914800, 120636000,
			136364400, 152085600, 167814000, 183535200, 199263600, 215589600, 230713200, 247039200,
			262767600, 278488800, 294217200, 309938400, 325666800, 341388000, 357116400, 372837600,
			388566000, 404892000, 420015600, 436341600, 452070000, 467791200, 483519600, 499240800,
			514969200, 530690400, 544604400, 562140000, 576054000, 594194400, 607503600, 625644000,
			638953200, 657093600, 671007600, 688543200, 702457200, 719992800, 733906800, 752047200,
			765356400, 783496800, 796806000, 814946400, 828860400, 846396000, 860310000, 877845600,
			891759600, 909295200, 923209200, 941349600, 954662400, 972802800, 986108400,
			1004248800, 1018162800, 1035698400, 1049612400, 1067148000, 1081062000, 1099202400,
			1112511600, 1130652000, 1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			4, 1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			5, 6, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["-00", "EPT", "EST", "EDT", "EWT", "CST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_PARAMARIBO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1861906760, -1104524348, -765317964, 465449400],
		transition_types: &[1, 2, 3, 4],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13240, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -13252, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -13236, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -12600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "PMT", "PMT", "-0330", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_PORT_AU_PRINCE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524504240, -1670483460, 421218000, 436334400, 452062800, 467784000, 483512400,
			499233600, 514962000, 530683200, 546411600, 562132800, 576050400, 594194400, 607500000,
			625644000, 638949600, 657093600, 671004000, 688543200, 702453600, 719992800, 733903200,
			752047200, 765352800, 783496800, 796802400, 814946400, 828856800, 846396000, 860306400,
			877845600, 1112504400, 1130644800, 1143954000, 1162094400, 1331449200, 1352008800,
			1362898800, 1383458400, 1394348400, 1414908000, 1425798000, 1446357600, 1489302000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -17360, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -17340, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 9 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "PPMT", "EDT", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_PORTO_VELHO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767210264, -1206954000, -1191358800, -1175371200, -1159822800, -633816000,
			-622065600, -602280000, -591829200, -570744000, -560206800, -539121600, -531349200,
			-191361600, -184194000, -155160000, -150066000, -128894400, -121122000, -99950400,
			-89586000, -68414400, -57963600, 499752000, 511239600, 530596800, 540270000, 562132800,
			571201200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15336, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-03", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_PUNTA_ARENAS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524504580, -1892661435, -1688410800, -1619205435, -1593806400, -1335986235,
			-1317585600, -1304362800, -1286049600, -1272826800, -1254513600, -1241290800,
			-1222977600, -1209754800, -1191355200, -1178132400, -870552000, -865278000, -736632000,
			-718056000, -713649600, -36619200, -23922000, -3355200, 7527600, 24465600, 37767600,
			55915200, 69217200, 87969600, 100666800, 118209600, 132116400, 150868800, 163566000,
			182318400, 195620400, 213768000, 227070000, 245217600, 258519600, 277272000, 289969200,
			308721600, 321418800, 340171200, 353473200, 371620800, 384922800, 403070400, 416372400,
			434520000, 447822000, 466574400, 479271600, 498024000, 510721200, 529473600, 545194800,
			560923200, 574225200, 592372800, 605674800, 624427200, 637124400, 653457600, 668574000,
			687326400, 700628400, 718776000, 732078000, 750225600, 763527600, 781675200, 794977200,
			813729600, 826426800, 845179200, 859690800, 876628800, 889930800, 906868800, 923194800,
			939528000, 952830000, 971582400, 984279600, 1003032000, 1015729200, 1034481600,
			1047178800, 1065931200, 1079233200, 1097380800, 1110682800, 1128830400, 1142132400,
			1160884800, 1173582000, 1192334400, 1206846000, 1223784000, 1237086000, 1255233600,
			1270350000, 1286683200, 1304823600, 1313899200, 1335668400, 1346558400, 1367118000,
			1378612800, 1398567600, 1410062400, 1463281200, 1471147200, 1480820400,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 3, 2, 3, 4, 2, 3, 5, 3, 5, 3, 5, 3, 5, 3,
			5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5,
			3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3,
			5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5,
			6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -17020, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -16965, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "SMT", "-05", "-04", "-04", "-03", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_RANKIN_INLET: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-410227200, 73468800, 89190000, 104918400, 120639600, 136368000, 152089200, 167817600,
			183538800, 199267200, 215593200, 230716800, 247042800, 262771200, 278492400, 294220800,
			309942000, 325670400, 341391600, 357120000, 372841200, 388569600, 404895600, 420019200,
			436345200, 452073600, 467794800, 483523200, 499244400, 514972800, 530694000, 544608000,
			562143600, 576057600, 594198000, 607507200, 625647600, 638956800, 657097200, 671011200,
			688546800, 702460800, 719996400, 733910400, 752050800, 765360000, 783500400, 796809600,
			814950000, 828864000, 846399600, 860313600, 877849200, 891763200, 909298800, 923212800,
			941353200, 954662400, 972802800, 986112000, 1004252400, 1018166400, 1035702000,
			1049616000, 1067151600, 1081065600, 1099206000, 1112515200, 1130655600, 1143964800,
			1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			3, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["-00", "CDT", "CST", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_RECIFE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767217224, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-191365200, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600, 592974000, 602042400, 624423600, 634701600, 938919600, 951616800, 970974000,
			971575200, 1003028400, 1013911200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -8376, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_RESOLUTE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-704937600, 73468800, 89190000, 104918400, 120639600, 136368000, 152089200, 167817600,
			183538800, 199267200, 215593200, 230716800, 247042800, 262771200, 278492400, 294220800,
			309942000, 325670400, 341391600, 357120000, 372841200, 388569600, 404895600, 420019200,
			436345200, 452073600, 467794800, 483523200, 499244400, 514972800, 530694000, 544608000,
			562143600, 576057600, 594198000, 607507200, 625647600, 638956800, 657097200, 671011200,
			688546800, 702460800, 719996400, 733910400, 752050800, 765360000, 783500400, 796809600,
			814950000, 828864000, 846399600, 860313600, 877849200, 891763200, 909298800, 923212800,
			941353200, 954662400, 972802800, 986112000, 1004252400, 1018166400, 1035702000,
			1049616000, 1067151600, 1081065600, 1099206000, 1112515200, 1130655600, 1143964800,
			1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			3, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["-00", "CDT", "CST", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_ROSARIO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2372096592, -1567453392, -1233432000, -1222981200, -1205956800, -1194037200,
			-1172865600, -1162501200, -1141329600, -1130965200, -1109793600, -1099429200,
			-1078257600, -1067806800, -1046635200, -1036270800, -1015099200, -1004734800,
			-983563200, -973198800, -952027200, -941576400, -931032000, -900882000, -890337600,
			-833749200, -827265600, -752274000, -733780800, -197326800, -190843200, -184194000,
			-164491200, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 596948400, 605066400,
			624423600, 636516000, 656478000, 667965600, 687931200, 699415200, 719377200, 731469600,
			938919600, 952052400, 1198983600, 1205632800, 1224385200, 1237082400,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 4, 5, 4, 2, 4, 5, 4, 5, 3, 5, 4,
			5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -15408, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "CMT", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_SANTAREM: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767212472, -1206954000, -1191358800, -1175371200, -1159822800, -633816000,
			-622065600, -602280000, -591829200, -570744000, -560206800, -539121600, -531349200,
			-191361600, -184194000, -155160000, -150066000, -128894400, -121122000, -99950400,
			-89586000, -68414400, -57963600, 499752000, 511239600, 530596800, 540270000, 562132800,
			571201200, 1214280000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13128, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-03", "-04", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AMERICA_SANTO_DOMINGO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524504824, -1159773600, -100119600, -89668800, -5770800, 4422600, 25678800, 33193800,
			57733200, 64816200, 89182800, 96438600, 120632400, 127974600, 152082000, 972799200,
			975823200,
		],
		transition_types: &[1, 3, 2, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 5, 3, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16776, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -16800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 9 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: -16200, is_dst: true, idx: 17 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 23 },
		],
		time_zone_designations: &["LMT", "SDMT", "EDT", "EST", "-0430", "AST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_SCORESBYSUND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1686090728, 323841600, 338961600, 354679200, 370400400, 386125200, 401850000,
			417574800, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400, 528253200,
			543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400,
			670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800,
			796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 2, 1, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -5272, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "-02", "-01", "-01", "+00"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-01", offset: 3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "+00", offset: 0 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 0 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 3600 },
		}),
	},
};
const AMERICA_SITKA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188954727, -880207200, -769395600, -765385200, -21477600, -5756400,
			9972000, 25693200, 41421600, 57747600, 73476000, 89197200, 104925600, 120646800,
			126698400, 152096400, 162381600, 183546000, 199274400, 215600400, 230724000, 247050000,
			262778400, 278499600, 294228000, 309949200, 325677600, 341398800, 357127200, 372848400,
			388576800, 404902800, 420026400, 436352400, 439030800, 452084400, 467805600, 483534000,
			499255200, 514983600, 530704800, 544618800, 562154400, 576068400, 594208800, 607518000,
			625658400, 638967600, 657108000, 671022000, 688557600, 702471600, 720007200, 733921200,
			752061600, 765370800, 783511200, 796820400, 814960800, 828874800, 846410400, 860324400,
			877860000, 891774000, 909309600, 923223600, 941364000, 954673200, 972813600, 986122800,
			1004263200, 1018177200, 1035712800, 1049626800, 1067162400, 1081076400, 1099216800,
			1112526000, 1130666400, 1143975600, 1162116000, 1173610800,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2,
			5, 2, 5, 2, 5, 6, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8,
			7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 53927, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -32473, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 24 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 29 },
		],
		time_zone_designations: &["LMT", "LMT", "PST", "PWT", "PPT", "PDT", "YST", "AKDT", "AKST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AKST", offset: 32400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AKDT", offset: 28800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_SWIFT_CURRENT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2030201320, -1632063600, -1615132800, -880210800, -769395600, -765388800, -747241200,
			-732729600, -715791600, -702489600, -684342000, -671040000, -652892400, -639590400,
			-400086000, -384364800, -337186800, -321465600, -305737200, -292435200, -273682800,
			-260985600, 73472400,
		],
		transition_types: &[2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25880, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_TEGUCIGALPA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1538503868, 547020000, 559717200, 578469600, 591166800, 1146981600, 1154926800,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20932, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const AMERICA_THULE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1686079492, 670399200, 686120400, 701848800, 717570000, 733903200, 752043600,
			765352800, 783493200, 796802400, 814942800, 828856800, 846392400, 860306400, 877842000,
			891756000, 909291600, 923205600, 941346000, 954655200, 972795600, 986104800,
			1004245200, 1018159200, 1035694800, 1049608800, 1067144400, 1081058400, 1099198800,
			1112508000, 1130648400, 1143957600, 1162098000, 1173592800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16508, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "ADT", "AST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ADT", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const AMERICA_VIRGIN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2233035335, -873057600, -769395600, -765399600],
		transition_types: &[1, 3, 2, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15865, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "AST", "APT", "AWT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: None,
	},
};
const AMERICA_YAKUTAT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188953665, -880203600, -769395600, -765381600, -21474000, -5752800,
			9975600, 25696800, 41425200, 57751200, 73479600, 89200800, 104929200, 120650400,
			126702000, 152100000, 162385200, 183549600, 199278000, 215604000, 230727600, 247053600,
			262782000, 278503200, 294231600, 309952800, 325681200, 341402400, 357130800, 372852000,
			388580400, 404906400, 420030000, 436356000, 439030800, 452084400, 467805600, 483534000,
			499255200, 514983600, 530704800, 544618800, 562154400, 576068400, 594208800, 607518000,
			625658400, 638967600, 657108000, 671022000, 688557600, 702471600, 720007200, 733921200,
			752061600, 765370800, 783511200, 796820400, 814960800, 828874800, 846410400, 860324400,
			877860000, 891774000, 909309600, 923223600, 941364000, 954673200, 972813600, 986122800,
			1004263200, 1018177200, 1035712800, 1049626800, 1067162400, 1081076400, 1099216800,
			1112526000, 1130666400, 1143975600, 1162116000, 1173610800,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2,
			5, 2, 5, 2, 5, 2, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7,
			6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 52865, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -33535, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 25 },
		],
		time_zone_designations: &["LMT", "LMT", "YST", "YWT", "YPT", "YDT", "AKDT", "AKST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AKST", offset: 32400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AKDT", offset: 28800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const ANTARCTICA_CASEY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-31536000, 1255802400, 1267714800, 1319738400, 1329843600, 1477065600, 1520701200,
			1538856000, 1552752000, 1570129200, 1583596800, 1601740860,
		],
		transition_types: &[1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["-00", "+08", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const ANTARCTICA_DAVIS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-409190400, -163062000, -28857600, 1255806000, 1268251200, 1319742000, 1329854400,
		],
		transition_types: &[1, 0, 1, 2, 1, 2, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["-00", "+07", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ANTARCTICA_MACQUARIE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2214259200, -1680508800, -1665388800, -1601719200, -687052800, -71136000, -55411200,
			-37267200, -25776000, -5817600, 5673600, 25632000, 37728000, 57686400, 67968000,
			89136000, 100022400, 120585600, 131472000, 152035200, 162921600, 183484800, 194976000,
			215539200, 226425600, 246988800, 257875200, 278438400, 289324800, 309888000, 320774400,
			341337600, 352224000, 372787200, 386092800, 404841600, 417542400, 436291200, 447177600,
			467740800, 478627200, 499190400, 510076800, 530035200, 542736000, 562089600, 574790400,
			594144000, 606240000, 625593600, 637689600, 657043200, 670348800, 686678400, 701798400,
			718128000, 733248000, 749577600, 764697600, 781027200, 796147200, 812476800, 828201600,
			844531200, 859651200, 875980800, 891100800, 907430400, 922550400, 938880000, 954000000,
			967305600, 985449600, 1002384000, 1017504000, 1033833600, 1048953600, 1065283200,
			1080403200, 1096732800, 1111852800, 1128182400, 1143907200, 1159632000, 1174752000,
			1191686400, 1207411200, 1223136000, 1238860800, 1254585600, 1301760000,
		],
		transition_types: &[
			1, 2, 1, 0, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 9 },
		],
		time_zone_designations: &["-00", "AEST", "AEDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AEST", offset: -36000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AEDT", offset: -39600 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const ANTARCTICA_MAWSON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-501206400, 1255809600],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["-00", "+06", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ANTARCTICA_PALMER: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-157766400, -152658000, -132955200, -121122000, -101419200, -86821200, -71092800,
			-54766800, -39038400, -23317200, -7588800, 128142000, 136605600, 389070000, 403070400,
			416372400, 434520000, 447822000, 466574400, 479271600, 498024000, 510721200, 529473600,
			545194800, 560923200, 574225200, 592372800, 605674800, 624427200, 637124400, 653457600,
			668574000, 687326400, 700628400, 718776000, 732078000, 750225600, 763527600, 781675200,
			794977200, 813729600, 826426800, 845179200, 859690800, 876628800, 889930800, 906868800,
			923194800, 939528000, 952830000, 971582400, 984279600, 1003032000, 1015729200,
			1034481600, 1047178800, 1065931200, 1079233200, 1097380800, 1110682800, 1128830400,
			1142132400, 1160884800, 1173582000, 1192334400, 1206846000, 1223784000, 1237086000,
			1255233600, 1270350000, 1286683200, 1304823600, 1313899200, 1335668400, 1346558400,
			1367118000, 1378612800, 1398567600, 1410062400, 1463281200, 1471147200, 1480820400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 4, 3, 4, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["-00", "-04", "-03", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const ANTARCTICA_ROTHERA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[218246400],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["-00", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const ANTARCTICA_TROLL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[1108166400, 1111885200],
		transition_types: &[2, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["-00", "+02", "+00"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+00", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "+02", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const ARCTIC_LONGYEARBYEN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2422054408, -1693706400, -1680483600, -1663455600, -1650150000, -1632006000,
			-1618700400, -938905200, -857257200, -844556400, -828226800, -812502000, -796777200,
			-781052400, -776563200, -765936000, -761180400, -748479600, -733273200, -717631200,
			-714610800, -710380800, -701910000, -684975600, -670460400, -654130800, -639010800,
			323830800, 338950800, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600,
			449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800,
			575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000,
			701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400,
			828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 2, 1, 2, 1, 3, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3208, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 13 },
		],
		time_zone_designations: &["LMT", "CEST", "CET", "CEMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const ASIA_ALMATY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441170468, -1247547600, 354909600, 370717200, 386445600, 402253200, 417981600,
			433789200, 449604000, 465336000, 481060800, 496785600, 512510400, 528235200, 543960000,
			559684800, 575409600, 591134400, 606859200, 622584000, 638308800, 654638400, 670363200,
			686091600, 695768400, 701812800, 717537600, 733262400, 748987200, 764712000, 780436800,
			796161600, 811886400, 828216000, 846360000, 859665600, 877809600, 891115200, 909259200,
			922564800, 941313600, 954014400, 972763200, 985464000, 1004212800, 1017518400,
			1035662400, 1048968000, 1067112000, 1080417600, 1099166400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 18468, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+05", "+07", "+06", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_AMMAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1230776624, 108165600, 118270800, 136591200, 149806800, 168127200, 181342800,
			199749600, 215643600, 231285600, 244501200, 262735200, 275950800, 481154400, 496962000,
			512949600, 528670800, 544399200, 560120400, 575848800, 592174800, 610581600, 623624400,
			641167200, 655074000, 671839200, 685918800, 702856800, 717973200, 733701600, 749422800,
			765151200, 779662800, 797205600, 811116000, 828655200, 843170400, 860104800, 874620000,
			891554400, 906069600, 930780000, 938124000, 954367200, 970178400, 985816800,
			1001628000, 1017352800, 1033077600, 1048802400, 1066946400, 1080252000, 1097791200,
			1112306400, 1128031200, 1143756000, 1161900000, 1175205600, 1193349600, 1206655200,
			1225404000, 1238104800, 1256853600, 1269554400, 1288303200, 1301608800, 1319752800,
			1333058400, 1387486800, 1395957600, 1414706400, 1427407200, 1446156000, 1459461600,
			1477605600, 1490911200, 1509055200, 1522360800, 1540504800, 1553810400, 1571954400,
			1585260000, 1604008800, 1616709600, 1635458400, 1645740000, 1666908000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8624, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "EEST", "EET", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const ASIA_ANADYR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441194596, -1247572800, 354884400, 370692000, 386420400, 402231600, 417960000,
			433767600, 449582400, 465314400, 481039200, 496764000, 512488800, 528213600, 543938400,
			559663200, 575388000, 591112800, 606837600, 622562400, 638287200, 654616800, 670341600,
			686070000, 695746800, 701791200, 717516000, 733240800, 748965600, 764690400, 780415200,
			796140000, 811864800, 828194400, 846338400, 859644000, 877788000, 891093600, 909237600,
			922543200, 941292000, 953992800, 972741600, 985442400, 1004191200, 1017496800,
			1035640800, 1048946400, 1067090400, 1080396000, 1099144800, 1111845600, 1130594400,
			1143295200, 1162044000, 1174744800, 1193493600, 1206799200, 1224943200, 1238248800,
			1256392800, 1269698400, 1288450800, 1301151600,
		],
		transition_types: &[
			1, 3, 2, 3, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 5, 6, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 1, 5, 6, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 42596, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 50400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+12", "+14", "+13", "+13", "+12", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const ASIA_AQTAU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441164064, -1247544000, 370724400, 386445600, 402256800, 417985200, 433792800,
			449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600, 559688400,
			575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800, 686095200,
			695772000, 701816400, 717541200, 733266000, 748990800, 764715600, 780440400, 796168800,
			811893600, 828223200, 846367200, 859672800, 877816800, 891122400, 909266400, 922572000,
			941320800, 954021600, 972770400, 985471200, 1004220000, 1017525600, 1035669600,
			1048975200, 1067119200, 1080424800, 1099173600,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 5, 1, 2, 4, 2, 4, 2, 4,
			1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 12064, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "+04", "+05", "+06", "+06", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_AQTOBE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441165720, -1247544000, 354913200, 370720800, 386445600, 402256800, 417985200,
			433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600,
			559688400, 575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800,
			686095200, 695772000, 701816400, 717541200, 733266000, 748990800, 764715600, 780440400,
			796165200, 811890000, 828219600, 846363600, 859669200, 877813200, 891118800, 909262800,
			922568400, 941317200, 954018000, 972766800, 985467600, 1004216400, 1017522000,
			1035666000, 1048971600, 1067115600, 1080421200, 1099170000,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 1, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 13720, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "+04", "+05", "+06", "+06", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_ASHKHABAD: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441166012, -1247544000, 354913200, 370720800, 386449200, 402256800, 417985200,
			433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600,
			559688400, 575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800,
			686095200, 695772000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 14012, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+04", "+06", "+05", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_ATYRAU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441164464, -1247540400, 370724400, 386445600, 402256800, 417985200, 433792800,
			449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600, 559688400,
			575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800, 686095200,
			695772000, 701816400, 717541200, 733266000, 748990800, 764715600, 780440400, 796165200,
			811890000, 828219600, 846363600, 859669200, 877813200, 891118800, 909262800, 922568400,
			941320800, 954021600, 972770400, 985471200, 1004220000, 1017525600, 1035669600,
			1048975200, 1067119200, 1080424800, 1099173600,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 5, 6, 2, 4, 2, 4, 2, 4,
			2, 4, 2, 4, 2, 4, 2, 4, 2, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 12464, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+06", "+06", "+05", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_BAGHDAD: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524532260, -1641005856, 389048400, 402264000, 417906000, 433800000, 449614800,
			465422400, 481150800, 496792800, 512517600, 528242400, 543967200, 559692000, 575416800,
			591141600, 606866400, 622591200, 638316000, 654645600, 670464000, 686275200, 702086400,
			717897600, 733622400, 749433600, 765158400, 780969600, 796694400, 812505600, 828316800,
			844128000, 859852800, 875664000, 891388800, 907200000, 922924800, 938736000, 954547200,
			970358400, 986083200, 1001894400, 1017619200, 1033430400, 1049155200, 1064966400,
			1080777600, 1096588800, 1112313600, 1128124800, 1143849600, 1159660800, 1175385600,
			1191196800,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 10660, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10656, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "BMT", "+03", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const ASIA_BAHRAIN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1577935568, 76190400],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 12368, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+04", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const ASIA_BAKU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441163964, -405140400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591141600, 606866400, 622591200, 638316000, 654645600, 670370400,
			686098800, 701823600, 717548400, 828234000, 846378000, 859680000, 877824000, 891129600,
			909273600, 922579200, 941328000, 954028800, 972777600, 985478400, 1004227200,
			1017532800, 1035676800, 1048982400, 1067126400, 1080432000, 1099180800, 1111881600,
			1130630400, 1143331200, 1162080000, 1174780800, 1193529600, 1206835200, 1224979200,
			1238284800, 1256428800, 1269734400, 1288483200, 1301184000, 1319932800, 1332633600,
			1351382400, 1364688000, 1382832000, 1396137600, 1414281600, 1427587200, 1445731200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 4, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 11964, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+04", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const ASIA_BARNAUL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1579844100, -1247551200, 354906000, 370713600, 386442000, 402249600, 417978000,
			433785600, 449600400, 465332400, 481057200, 496782000, 512506800, 528231600, 543956400,
			559681200, 575406000, 591130800, 606855600, 622580400, 638305200, 654634800, 670359600,
			686088000, 695764800, 701809200, 717534000, 733258800, 748983600, 764708400, 780433200,
			796158000, 801590400, 811886400, 828216000, 846360000, 859665600, 877809600, 891115200,
			909259200, 922564800, 941313600, 954014400, 972763200, 985464000, 1004212800,
			1017518400, 1035662400, 1048968000, 1067112000, 1080417600, 1099166400, 1111867200,
			1130616000, 1143316800, 1162065600, 1174766400, 1193515200, 1206820800, 1224964800,
			1238270400, 1256414400, 1269720000, 1288468800, 1301169600, 1414263600, 1459022400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 3, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 20100, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+06", "+08", "+07", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_BEIRUT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840149320, -1570413600, -1552186800, -1538359200, -1522551600, -1507514400,
			-1490583600, -1473645600, -1460948400, -399866400, -386650800, -368330400, -355114800,
			-336794400, -323578800, -305172000, -291956400, -273636000, -260420400, 78012000,
			86734800, 105055200, 118270800, 136591200, 149806800, 168127200, 181342800, 199749600,
			212965200, 231285600, 244501200, 262735200, 275950800, 452210400, 466722000, 483746400,
			498258000, 515282400, 529794000, 546818400, 561330000, 581119200, 592952400, 610754400,
			624488400, 641512800, 656024400, 673048800, 687560400, 704671200, 718146000, 733269600,
			748990800, 764719200, 780440400, 796168800, 811890000, 828223200, 843944400, 859672800,
			875394000, 891122400, 906843600, 922572000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8520, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "EEST", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 0 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 0 },
		}),
	},
};
const ASIA_BISHKEK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441169904, -1247547600, 354909600, 370717200, 386445600, 402253200, 417981600,
			433789200, 449604000, 465336000, 481060800, 496785600, 512510400, 528235200, 543960000,
			559684800, 575409600, 591134400, 606859200, 622584000, 638308800, 654638400, 670363200,
			683582400, 703018800, 717530400, 734468400, 748980000, 765918000, 780429600, 797367600,
			811879200, 828817200, 843933600, 859671000, 877811400, 891120600, 909261000, 922570200,
			941315400, 954019800, 972765000, 985469400, 1004214600, 1017523800, 1035664200,
			1048973400, 1067113800, 1080423000, 1099168200, 1111872600, 1123783200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 17904, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+05", "+07", "+06", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_BRUNEI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1383463280, -1167636600, -1082448000, -1074586800, -1050825600, -1042964400,
			-1019289600, -1011428400, -987753600, -979892400, -956217600, -948356400, -924595200,
			-916734000, -893059200, -885198000, -879667200, -767005200,
		],
		transition_types: &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 26480, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 27000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 30000, is_dst: true, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "+0730", "+0820", "+08", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+08", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_CALCUTTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3645237208, -3155694800, -2019705670, -891581400, -872058600, -862637400, -764145000,
		],
		transition_types: &[1, 2, 3, 4, 3, 4, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 21208, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 19270, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 19800, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 23400, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "HMT", "MMT", "IST", "+0630"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "IST", offset: -19800 },
		dst_info: None,
	},
};
const ASIA_CHITA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1579419232, -1247558400, 354898800, 370706400, 386434800, 402242400, 417970800,
			433778400, 449593200, 465325200, 481050000, 496774800, 512499600, 528224400, 543949200,
			559674000, 575398800, 591123600, 606848400, 622573200, 638298000, 654627600, 670352400,
			686080800, 695757600, 701802000, 717526800, 733251600, 748976400, 764701200, 780426000,
			796150800, 811875600, 828205200, 846349200, 859654800, 877798800, 891104400, 909248400,
			922554000, 941302800, 954003600, 972752400, 985453200, 1004202000, 1017507600,
			1035651600, 1048957200, 1067101200, 1080406800, 1099155600, 1111856400, 1130605200,
			1143306000, 1162054800, 1174755600, 1193504400, 1206810000, 1224954000, 1238259600,
			1256403600, 1269709200, 1288458000, 1301158800, 1414252800, 1459015200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 27232, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+08", "+10", "+09", "+09", "+10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+09", offset: -32400 },
		dst_info: None,
	},
};
const ASIA_CHOIBALSAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2032933080, 252435600, 417974400, 433778400, 449593200, 465314400, 481042800,
			496764000, 512492400, 528213600, 543942000, 559663200, 575391600, 591112800, 606841200,
			622562400, 638290800, 654616800, 670345200, 686066400, 701794800, 717516000, 733244400,
			748965600, 764694000, 780415200, 796143600, 811864800, 828198000, 843919200, 859647600,
			875368800, 891097200, 906818400, 988390800, 1001692800, 1017421200, 1033142400,
			1048870800, 1064592000, 1080320400, 1096041600, 1111770000, 1127491200, 1143219600,
			1159545600, 1206889200, 1427479200, 1443193200, 1458928800, 1474642800,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 2, 5, 2, 5, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 27480, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+07", "+08", "+09", "+10", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+08", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_COLOMBO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840159964, -2019705572, -883287000, -862639200, -764051400, 832962600, 846266400,
			1145039400,
		],
		transition_types: &[1, 2, 3, 4, 2, 5, 6, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 19164, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 19172, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 19800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 23400, is_dst: true, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 23400, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "MMT", "+0530", "+06", "+0630", "+0630", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+0530", offset: -19800 },
		dst_info: None,
	},
};
const ASIA_DACCA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524543300, -891582800, -872058600, -862637400, -576138600, 1245430800, 1262278800,
		],
		transition_types: &[1, 2, 3, 2, 4, 5, 4],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 21700, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 23400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 19800, is_dst: false, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "HMT", "+0630", "+0530", "+06", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_DAMASCUS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1577931912, -1568592000, -1554080400, -1537142400, -1522630800, -1505692800,
			-1491181200, -1474243200, -1459126800, -242265600, -228877200, -210556800, -197427600,
			-178934400, -165718800, -147398400, -134269200, -116467200, -102646800, -84326400,
			-71110800, -52704000, -39488400, -21168000, -7952400, 10368000, 23583600, 41904000,
			55119600, 73526400, 86742000, 105062400, 118278000, 136598400, 149814000, 168134400,
			181350000, 199756800, 212972400, 231292800, 241916400, 262828800, 273452400, 418694400,
			433810800, 450316800, 465433200, 508896000, 529196400, 541555200, 562633200, 574387200,
			594255600, 607305600, 623199600, 638928000, 654649200, 670456800, 686264400, 702684000,
			717886800, 733096800, 748904400, 765151200, 780958800, 796687200, 812494800, 828309600,
			844117200, 859759200, 875653200, 891208800, 907189200, 922917600, 938725200, 954540000,
			970347600, 986076000, 1001883600, 1017612000, 1033419600, 1049148000, 1064955600,
			1080770400, 1096578000, 1112306400, 1128114000, 1143842400, 1158872400, 1175205600,
			1193950800, 1207260000, 1225486800, 1238104800, 1256850000, 1270159200, 1288299600,
			1301608800, 1319749200, 1333058400, 1351198800, 1364508000, 1382648400, 1395957600,
			1414702800, 1427407200, 1446152400, 1458856800, 1477602000, 1490911200, 1509051600,
			1522360800, 1540501200, 1553810400, 1571950800, 1585260000, 1604005200, 1616709600,
			1635454800, 1648159200, 1666904400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8712, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "EEST", "EET", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const ASIA_DILI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1830414140, -879152400, 199897200, 969120000],
		transition_types: &[1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 30140, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+08", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+09", offset: -32400 },
		dst_info: None,
	},
};
const ASIA_DUSHANBE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441168512, -1247547600, 354909600, 370717200, 386445600, 402253200, 417981600,
			433789200, 449604000, 465336000, 481060800, 496785600, 512510400, 528235200, 543960000,
			559684800, 575409600, 591134400, 606859200, 622584000, 638308800, 654638400, 670363200,
			684363600,
		],
		transition_types: &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 16512, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+05", "+07", "+06", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_FAMAGUSTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1518920148, 166572000, 182293200, 200959200, 213829200, 228866400, 243982800,
			260316000, 276123600, 291765600, 307486800, 323820000, 338936400, 354664800, 370386000,
			386114400, 401835600, 417564000, 433285200, 449013600, 465339600, 481068000, 496789200,
			512517600, 528238800, 543967200, 559688400, 575416800, 591138000, 606866400, 622587600,
			638316000, 654642000, 670370400, 686091600, 701820000, 717541200, 733269600, 748990800,
			764719200, 780440400, 796168800, 811890000, 828223200, 843944400, 859672800, 875394000,
			891122400, 909277200, 922582800, 941331600, 954032400, 972781200, 985482000,
			1004230800, 1017536400, 1035680400, 1048986000, 1067130000, 1080435600, 1099184400,
			1111885200, 1130634000, 1143334800, 1162083600, 1174784400, 1193533200, 1206838800,
			1224982800, 1238288400, 1256432400, 1269738000, 1288486800, 1301187600, 1319936400,
			1332637200, 1351386000, 1364691600, 1382835600, 1396141200, 1414285200, 1427590800,
			1445734800, 1459040400, 1473282000, 1509238800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8148, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "EEST", "EET", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const ASIA_GAZA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2185409872, -933638400, -923097600, -919036800, -857347200, -844300800, -825811200,
			-812678400, -794188800, -779846400, -762652800, -748310400, -731116800, -399088800,
			-386650800, -368330400, -355114800, -336790800, -323654400, -305168400, -292032000,
			-273632400, -260496000, -242096400, -228960000, -210560400, -197424000, -178938000,
			-165801600, -147402000, -134265600, -115866000, -102643200, -84330000, -81313200,
			142380000, 150843600, 167176800, 178664400, 334101600, 337730400, 452642400, 462319200,
			482277600, 494370000, 516751200, 526424400, 545436000, 558478800, 576626400, 589323600,
			609890400, 620773200, 638316000, 651618000, 669765600, 683672400, 701820000, 715726800,
			733701600, 747176400, 765151200, 778021200, 796600800, 810075600, 820447200, 828655200,
			843170400, 860104800, 874620000, 891554400, 906069600, 924213600, 939934800, 956268000,
			971989200, 987717600, 1003438800, 1019167200, 1034888400, 1050616800, 1066338000,
			1082066400, 1096581600, 1113516000, 1128380400, 1143842400, 1158872400, 1175378400,
			1189638000, 1206655200, 1219957200, 1238104800, 1252015200, 1269640860, 1281474000,
			1301608860, 1312146000, 1333058400, 1348178400, 1364508000, 1380229200, 1395957600,
			1414098000, 1427493600, 1445551200, 1458946800, 1477692000, 1490396400, 1509141600,
			1521846000, 1540591200, 1553810400, 1572037200, 1585346400, 1603490400, 1616796000,
			1635458400, 1648332000, 1666998000, 1682726400, 1698447600, 1712966400, 1729897200,
			1743811200, 1761346800, 1774656000, 1792796400, 1806105600, 1824850800, 1837555200,
			1856300400, 1869004800, 1887750000, 1901059200, 1919199600, 1932508800, 1950649200,
			1963958400, 1982703600, 1995408000, 2014153200, 2026857600, 2045602800, 2058307200,
			2077052400, 2090361600, 2107897200, 2121811200, 2138742000, 2153260800, 2168982000,
			2184710400, 2199826800, 2202854400, 2203455600, 2216160000, 2230066800, 2233699200,
			2234905200, 2248214400, 2260911600, 2263939200, 2266354800, 2279664000, 2291756400,
			2294784000, 2297804400, 2311113600, 2321996400, 2325628800, 2329254000, 2342563200,
			2352841200, 2355868800, 2361308400, 2374012800, 2383686000, 2386713600, 2392758000,
			2405462400, 2413926000, 2417558400, 2424207600, 2437516800, 2444770800, 2447798400,
			2455657200, 2468966400, 2475010800, 2478643200, 2487106800, 2500416000, 2505855600,
			2508883200, 2519161200, 2531865600, 2536700400, 2539728000, 2550610800, 2563315200,
			2566940400, 2570572800, 2582060400, 2595369600, 2597785200, 2600812800, 2613510000,
			2626819200, 2628025200, 2631657600, 2644959600, 2658268800, 2658870000, 2662502400,
			2676409200, 2692742400, 2708463600, 2723587200, 2739913200, 2753827200, 2771362800,
			2784672000, 2802812400, 2816121600, 2834262000, 2847571200, 2866316400, 2879020800,
			2897766000, 2910470400, 2929215600, 2941920000, 2960665200, 2973974400, 2992114800,
			3005424000, 3023564400, 3036873600, 3055618800, 3068323200, 3087068400, 3099772800,
			3117913200, 3131827200, 3148758000, 3163276800, 3179602800, 3194726400, 3209842800,
			3226176000, 3240687600, 3243715200, 3271532400, 3274560000, 3301772400, 3305404800,
			3332617200, 3335644800, 3339270000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8272, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 17 },
		],
		time_zone_designations: &["LMT", "EEST", "EET", "IDT", "IST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 4, 4), time: 180000 },
			end_date: TransitionDate { day: MonthWeekDay(10, 4, 4), time: 180000 },
		}),
	},
};
const ASIA_HEBRON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2185410023, -933638400, -923097600, -919036800, -857347200, -844300800, -825811200,
			-812678400, -794188800, -779846400, -762652800, -748310400, -731116800, -399088800,
			-386650800, -368330400, -355114800, -336790800, -323654400, -305168400, -292032000,
			-273632400, -260496000, -242096400, -228960000, -210560400, -197424000, -178938000,
			-165801600, -147402000, -134265600, -115866000, -102643200, -84330000, -81313200,
			142380000, 150843600, 167176800, 178664400, 334101600, 337730400, 452642400, 462319200,
			482277600, 494370000, 516751200, 526424400, 545436000, 558478800, 576626400, 589323600,
			609890400, 620773200, 638316000, 651618000, 669765600, 683672400, 701820000, 715726800,
			733701600, 747176400, 765151200, 778021200, 796600800, 810075600, 820447200, 828655200,
			843170400, 860104800, 874620000, 891554400, 906069600, 924213600, 939934800, 956268000,
			971989200, 987717600, 1003438800, 1019167200, 1034888400, 1050616800, 1066338000,
			1082066400, 1096581600, 1113516000, 1128380400, 1143842400, 1158872400, 1175378400,
			1189638000, 1206655200, 1220216400, 1238104800, 1252015200, 1269554400, 1281474000,
			1301608860, 1312146000, 1314655200, 1317330000, 1333058400, 1348178400, 1364508000,
			1380229200, 1395957600, 1414098000, 1427493600, 1445551200, 1458946800, 1477692000,
			1490396400, 1509141600, 1521846000, 1540591200, 1553810400, 1572037200, 1585346400,
			1603490400, 1616796000, 1635458400, 1648332000, 1666998000, 1682726400, 1698447600,
			1712966400, 1729897200, 1743811200, 1761346800, 1774656000, 1792796400, 1806105600,
			1824850800, 1837555200, 1856300400, 1869004800, 1887750000, 1901059200, 1919199600,
			1932508800, 1950649200, 1963958400, 1982703600, 1995408000, 2014153200, 2026857600,
			2045602800, 2058307200, 2077052400, 2090361600, 2107897200, 2121811200, 2138742000,
			2153260800, 2168982000, 2184710400, 2199826800, 2202854400, 2203455600, 2216160000,
			2230066800, 2233699200, 2234905200, 2248214400, 2260911600, 2263939200, 2266354800,
			2279664000, 2291756400, 2294784000, 2297804400, 2311113600, 2321996400, 2325628800,
			2329254000, 2342563200, 2352841200, 2355868800, 2361308400, 2374012800, 2383686000,
			2386713600, 2392758000, 2405462400, 2413926000, 2417558400, 2424207600, 2437516800,
			2444770800, 2447798400, 2455657200, 2468966400, 2475010800, 2478643200, 2487106800,
			2500416000, 2505855600, 2508883200, 2519161200, 2531865600, 2536700400, 2539728000,
			2550610800, 2563315200, 2566940400, 2570572800, 2582060400, 2595369600, 2597785200,
			2600812800, 2613510000, 2626819200, 2628025200, 2631657600, 2644959600, 2658268800,
			2658870000, 2662502400, 2676409200, 2692742400, 2708463600, 2723587200, 2739913200,
			2753827200, 2771362800, 2784672000, 2802812400, 2816121600, 2834262000, 2847571200,
			2866316400, 2879020800, 2897766000, 2910470400, 2929215600, 2941920000, 2960665200,
			2973974400, 2992114800, 3005424000, 3023564400, 3036873600, 3055618800, 3068323200,
			3087068400, 3099772800, 3117913200, 3131827200, 3148758000, 3163276800, 3179602800,
			3194726400, 3209842800, 3226176000, 3240687600, 3243715200, 3271532400, 3274560000,
			3301772400, 3305404800, 3332617200, 3335644800, 3339270000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8423, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 17 },
		],
		time_zone_designations: &["LMT", "EEST", "EET", "IDT", "IST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 4, 4), time: 180000 },
			end_date: TransitionDate { day: MonthWeekDay(10, 4, 4), time: 180000 },
		}),
	},
};
const ASIA_HOVD: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2032927596, 252439200, 417978000, 433785600, 449600400, 465321600, 481050000,
			496771200, 512499600, 528220800, 543949200, 559670400, 575398800, 591120000, 606848400,
			622569600, 638298000, 654624000, 670352400, 686073600, 701802000, 717523200, 733251600,
			748972800, 764701200, 780422400, 796150800, 811872000, 828205200, 843926400, 859654800,
			875376000, 891104400, 906825600, 988398000, 1001700000, 1017428400, 1033149600,
			1048878000, 1064599200, 1080327600, 1096048800, 1111777200, 1127498400, 1143226800,
			1159552800, 1427482800, 1443196800, 1458932400, 1474646400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 21996, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "+06", "+08", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_IRKUTSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840165825, -1575874625, -1247554800, 354902400, 370710000, 386438400, 402246000,
			417974400, 433782000, 449596800, 465328800, 481053600, 496778400, 512503200, 528228000,
			543952800, 559677600, 575402400, 591127200, 606852000, 622576800, 638301600, 654631200,
			670356000, 686084400, 695761200, 701805600, 717530400, 733255200, 748980000, 764704800,
			780429600, 796154400, 811879200, 828208800, 846352800, 859658400, 877802400, 891108000,
			909252000, 922557600, 941306400, 954007200, 972756000, 985456800, 1004205600,
			1017511200, 1035655200, 1048960800, 1067104800, 1080410400, 1099159200, 1111860000,
			1130608800, 1143309600, 1162058400, 1174759200, 1193508000, 1206813600, 1224957600,
			1238263200, 1256407200, 1269712800, 1288461600, 1301162400, 1414256400,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 5, 2, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 6, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 25025, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 25025, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "IMT", "+07", "+09", "+08", "+08", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+08", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_JAKARTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3231299232, -1451719200, -1172906400, -876641400, -766054800, -683883000, -620812800,
			-189415800,
		],
		transition_types: &[1, 2, 3, 4, 3, 5, 3, 6],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 25632, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 25632, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 26400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 27000, is_dst: false, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 28 },
		],
		time_zone_designations: &["LMT", "BMT", "+0720", "+0730", "+09", "+08", "WIB"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WIB", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_JAYAPURA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1172913768, -799491600, -189423000],
		transition_types: &[1, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 33768, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 34200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "+09", "+0930", "WIT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WIT", offset: -32400 },
		dst_info: None,
	},
};
const ASIA_KABUL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524538208, -788932800],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 16608, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 16200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+04", "+0430"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+0430", offset: -16200 },
		dst_info: None,
	},
};
const ASIA_KAMCHATKA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1487759676, -1247569200, 354888000, 370695600, 386424000, 402231600, 417960000,
			433767600, 449582400, 465314400, 481039200, 496764000, 512488800, 528213600, 543938400,
			559663200, 575388000, 591112800, 606837600, 622562400, 638287200, 654616800, 670341600,
			686070000, 695746800, 701791200, 717516000, 733240800, 748965600, 764690400, 780415200,
			796140000, 811864800, 828194400, 846338400, 859644000, 877788000, 891093600, 909237600,
			922543200, 941292000, 953992800, 972741600, 985442400, 1004191200, 1017496800,
			1035640800, 1048946400, 1067090400, 1080396000, 1099144800, 1111845600, 1130594400,
			1143295200, 1162044000, 1174744800, 1193493600, 1206799200, 1224943200, 1238248800,
			1256392800, 1269698400, 1288450800, 1301151600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 4, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 38076, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+11", "+13", "+12", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const ASIA_KARACHI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1988166492, -862637400, -764145000, -576135000, 38775600, 1018119600, 1033840800,
			1212260400, 1225476000, 1239735600, 1257012000,
		],
		transition_types: &[1, 2, 1, 3, 5, 4, 5, 4, 5, 4, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 16092, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 19800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 23400, is_dst: true, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 25 },
		],
		time_zone_designations: &["LMT", "+0530", "+0630", "+05", "PKST", "PKT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "PKT", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_KASHGAR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1325483420],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 21020, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_KATMANDU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1577943676, 504901800],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 20476, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 19800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 20700, is_dst: false, idx: 10 },
		],
		time_zone_designations: &["LMT", "+0530", "+0545"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+0545", offset: -20700 },
		dst_info: None,
	},
};
const ASIA_KHANDYGA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1579424533, -1247558400, 354898800, 370706400, 386434800, 402242400, 417970800,
			433778400, 449593200, 465325200, 481050000, 496774800, 512499600, 528224400, 543949200,
			559674000, 575398800, 591123600, 606848400, 622573200, 638298000, 654627600, 670352400,
			686080800, 695757600, 701802000, 717526800, 733251600, 748976400, 764701200, 780426000,
			796150800, 811875600, 828205200, 846349200, 859654800, 877798800, 891104400, 909248400,
			922554000, 941302800, 954003600, 972752400, 985453200, 1004202000, 1017507600,
			1035651600, 1048957200, 1067101200, 1072882800, 1080403200, 1099152000, 1111852800,
			1130601600, 1143302400, 1162051200, 1174752000, 1193500800, 1206806400, 1224950400,
			1238256000, 1256400000, 1269705600, 1288454400, 1301155200, 1315832400, 1414252800,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 7, 6, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 32533, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+08", "+10", "+09", "+09", "+11", "+10", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+09", offset: -32400 },
		dst_info: None,
	},
};
const ASIA_KRASNOYARSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1577513486, -1247551200, 354906000, 370713600, 386442000, 402249600, 417978000,
			433785600, 449600400, 465332400, 481057200, 496782000, 512506800, 528231600, 543956400,
			559681200, 575406000, 591130800, 606855600, 622580400, 638305200, 654634800, 670359600,
			686088000, 695764800, 701809200, 717534000, 733258800, 748983600, 764708400, 780433200,
			796158000, 811882800, 828212400, 846356400, 859662000, 877806000, 891111600, 909255600,
			922561200, 941310000, 954010800, 972759600, 985460400, 1004209200, 1017514800,
			1035658800, 1048964400, 1067108400, 1080414000, 1099162800, 1111863600, 1130612400,
			1143313200, 1162062000, 1174762800, 1193511600, 1206817200, 1224961200, 1238266800,
			1256410800, 1269716400, 1288465200, 1301166000, 1414260000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 22286, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+06", "+08", "+07", "+07", "+08"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_KUWAIT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-719636812],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 11212, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const ASIA_MACAO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2056692850, -884509200, -873280800, -855918000, -841744800, -828529200, -765363600,
			-747046800, -733827600, -716461200, -697021200, -683715600, -667990800, -654771600,
			-636627600, -623322000, -605178000, -591872400, -573642000, -559818000, -541674000,
			-528368400, -510224400, -498128400, -478774800, -466678800, -446720400, -435229200,
			-415258200, -403158600, -383808600, -371709000, -352359000, -340259400, -320909400,
			-308809800, -288855000, -277360200, -257405400, -245910600, -225955800, -213856200,
			-194506200, -182406600, -163056600, -148537800, -132820200, -117088200, -101370600,
			-85638600, -69312600, -53584200, -37863000, -22134600, -6413400, 9315000, 25036200,
			40764600, 56485800, 72214200, 88540200, 104268600, 119989800, 126041400, 151439400,
			167167800, 182889000, 198617400, 214338600, 295385400, 309292200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 27250, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "CST", "+10", "+09", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_MAGADAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441188192, -1247565600, 354891600, 370699200, 386427600, 402235200, 417963600,
			433771200, 449586000, 465318000, 481042800, 496767600, 512492400, 528217200, 543942000,
			559666800, 575391600, 591116400, 606841200, 622566000, 638290800, 654620400, 670345200,
			686073600, 695750400, 701794800, 717519600, 733244400, 748969200, 764694000, 780418800,
			796143600, 811868400, 828198000, 846342000, 859647600, 877791600, 891097200, 909241200,
			922546800, 941295600, 953996400, 972745200, 985446000, 1004194800, 1017500400,
			1035644400, 1048950000, 1067094000, 1080399600, 1099148400, 1111849200, 1130598000,
			1143298800, 1162047600, 1174748400, 1193497200, 1206802800, 1224946800, 1238252400,
			1256396400, 1269702000, 1288450800, 1301151600, 1414245600, 1461427200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 36192, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+10", "+12", "+11", "+11", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const ASIA_MANILA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3944621040, -2229321840, -1046678400, -1038733200, -873273600, -794221200, -496224000,
			-489315600, 259344000, 275151600,
		],
		transition_types: &[1, 3, 2, 3, 4, 3, 2, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -57360, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 29040, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "LMT", "PDT", "PST", "JST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "PST", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_MUSCAT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1577936472],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 13272, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const ASIA_NOVOKUZNETSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441259328, -1247551200, 354906000, 370713600, 386442000, 402249600, 417978000,
			433785600, 449600400, 465332400, 481057200, 496782000, 512506800, 528231600, 543956400,
			559681200, 575406000, 591130800, 606855600, 622580400, 638305200, 654634800, 670359600,
			686088000, 695764800, 701809200, 717534000, 733258800, 748983600, 764708400, 780433200,
			796158000, 811882800, 828212400, 846356400, 859662000, 877806000, 891111600, 909255600,
			922561200, 941310000, 954010800, 972759600, 985460400, 1004209200, 1017514800,
			1035658800, 1048964400, 1067108400, 1080414000, 1099162800, 1111863600, 1130612400,
			1143313200, 1162062000, 1174762800, 1193511600, 1206817200, 1224961200, 1238266800,
			1256410800, 1269716400, 1288468800, 1301169600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 4, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 20928, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+06", "+08", "+07", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_NOVOSIBIRSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1579476700, -1247551200, 354906000, 370713600, 386442000, 402249600, 417978000,
			433785600, 449600400, 465332400, 481057200, 496782000, 512506800, 528231600, 543956400,
			559681200, 575406000, 591130800, 606855600, 622580400, 638305200, 654634800, 670359600,
			686088000, 695764800, 701809200, 717534000, 733258800, 738086400, 748987200, 764712000,
			780436800, 796161600, 811886400, 828216000, 846360000, 859665600, 877809600, 891115200,
			909259200, 922564800, 941313600, 954014400, 972763200, 985464000, 1004212800,
			1017518400, 1035662400, 1048968000, 1067112000, 1080417600, 1099166400, 1111867200,
			1130616000, 1143316800, 1162065600, 1174766400, 1193515200, 1206820800, 1224964800,
			1238270400, 1256414400, 1269720000, 1288468800, 1301169600, 1414263600, 1469304000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 4,
			1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 3, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 19900, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+06", "+08", "+07", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_OMSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1582088010, -1247547600, 354909600, 370717200, 386445600, 402253200, 417981600,
			433789200, 449604000, 465336000, 481060800, 496785600, 512510400, 528235200, 543960000,
			559684800, 575409600, 591134400, 606859200, 622584000, 638308800, 654638400, 670363200,
			686091600, 695768400, 701812800, 717537600, 733262400, 748987200, 764712000, 780436800,
			796161600, 811886400, 828216000, 846360000, 859665600, 877809600, 891115200, 909259200,
			922564800, 941313600, 954014400, 972763200, 985464000, 1004212800, 1017518400,
			1035662400, 1048968000, 1067112000, 1080417600, 1099166400, 1111867200, 1130616000,
			1143316800, 1162065600, 1174766400, 1193515200, 1206820800, 1224964800, 1238270400,
			1256414400, 1269720000, 1288468800, 1301169600, 1414263600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 17610, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+05", "+07", "+06", "+06", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_ORAL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441164324, -1247540400, 354913200, 370720800, 386445600, 402256800, 417985200,
			433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600,
			559688400, 575413200, 591138000, 606862800, 622591200, 638316000, 654645600, 670370400,
			686095200, 695772000, 701816400, 717544800, 733269600, 748994400, 764719200, 780444000,
			796168800, 811893600, 828223200, 846367200, 859672800, 877816800, 891122400, 909266400,
			922572000, 941320800, 954021600, 972770400, 985471200, 1004220000, 1017525600,
			1035669600, 1048975200, 1067119200, 1080424800, 1099173600,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 6, 5, 6, 5, 6, 2, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 12324, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+06", "+06", "+05", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_PONTIANAK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1946186240, -1172906240, -881220600, -766054800, -683883000, -620812800, -189415800,
			567964800,
		],
		transition_types: &[1, 2, 3, 2, 4, 2, 5, 6],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 26240, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 26240, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 27000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 22 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 27 },
		],
		time_zone_designations: &["LMT", "PMT", "+0730", "+09", "+08", "WITA", "WIB"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WIB", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_PYONGYANG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1948782180, -1830414600, -768646800, 1439564400, 1525446000],
		transition_types: &[1, 2, 3, 1, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 30180, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 30600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "KST", "JST", "KST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "KST", offset: -32400 },
		dst_info: None,
	},
};
const ASIA_QOSTANAY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441167268, -1247544000, 354913200, 370720800, 386445600, 402256800, 417985200,
			433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600,
			559688400, 575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800,
			686095200, 695772000, 701816400, 717541200, 733266000, 748990800, 764715600, 780440400,
			796165200, 811890000, 828219600, 846363600, 859669200, 877813200, 891118800, 909262800,
			922568400, 941317200, 954018000, 972766800, 985467600, 1004216400, 1017522000,
			1035666000, 1048971600, 1067115600, 1080421200, 1099170000,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 1, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 15268, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "+04", "+05", "+06", "+06", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_QYZYLORDA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441167712, -1247544000, 354913200, 370720800, 386445600, 402256800, 417985200,
			433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600,
			559688400, 575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800,
			686095200, 695768400, 701812800, 717541200, 733266000, 748990800, 764715600, 780440400,
			796165200, 811890000, 828219600, 846363600, 859669200, 877813200, 891118800, 909262800,
			922568400, 941317200, 954018000, 972766800, 985467600, 1004216400, 1017522000,
			1035666000, 1048971600, 1067115600, 1080421200, 1099170000, 1545328800,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 2, 4, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 15712, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "+04", "+05", "+06", "+06", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_RANGOON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2840163887, -1577946287, -873268200, -778410000],
		transition_types: &[1, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 23087, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 23087, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 23400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "RMT", "+0630", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+0630", offset: -23400 },
		dst_info: None,
	},
};
const ASIA_SAIGON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2004073590, -1851577590, -852105600, -782643600, -767869200, -718095600, -457776000,
			-315648000, 171820800,
		],
		transition_types: &[1, 2, 3, 4, 2, 3, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 25590, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 25590, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 17 },
		],
		time_zone_designations: &["LMT", "PLMT", "+07", "+08", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_SAKHALIN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2031039048, -768560400, 354891600, 370699200, 386427600, 402235200, 417963600,
			433771200, 449586000, 465318000, 481042800, 496767600, 512492400, 528217200, 543942000,
			559666800, 575391600, 591116400, 606841200, 622566000, 638290800, 654620400, 670345200,
			686073600, 695750400, 701794800, 717519600, 733244400, 748969200, 764694000, 780418800,
			796143600, 811868400, 828198000, 846342000, 859647600, 877795200, 891100800, 909244800,
			922550400, 941299200, 954000000, 972748800, 985449600, 1004198400, 1017504000,
			1035648000, 1048953600, 1067097600, 1080403200, 1099152000, 1111852800, 1130601600,
			1143302400, 1162051200, 1174752000, 1193500800, 1206806400, 1224950400, 1238256000,
			1256400000, 1269705600, 1288454400, 1301155200, 1414249200, 1459008000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 34248, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+09", "+12", "+11", "+11", "+10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const ASIA_SAMARKAND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441168073, -1247544000, 354913200, 370720800, 386445600, 402256800, 417985200,
			433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800, 543963600,
			559688400, 575413200, 591138000, 606862800, 622587600, 638312400, 654642000, 670366800,
			686091600,
		],
		transition_types: &[1, 2, 3, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 16073, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "+04", "+05", "+06", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_SREDNEKOLYMSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441188892, -1247565600, 354891600, 370699200, 386427600, 402235200, 417963600,
			433771200, 449586000, 465318000, 481042800, 496767600, 512492400, 528217200, 543942000,
			559666800, 575391600, 591116400, 606841200, 622566000, 638290800, 654620400, 670345200,
			686073600, 695750400, 701794800, 717519600, 733244400, 748969200, 764694000, 780418800,
			796143600, 811868400, 828198000, 846342000, 859647600, 877791600, 891097200, 909241200,
			922546800, 941295600, 953996400, 972745200, 985446000, 1004194800, 1017500400,
			1035644400, 1048950000, 1067094000, 1080399600, 1099148400, 1111849200, 1130598000,
			1143298800, 1162047600, 1174748400, 1193497200, 1206802800, 1224946800, 1238252400,
			1256396400, 1269702000, 1288450800, 1301151600, 1414245600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 36892, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+10", "+12", "+11", "+11", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const ASIA_TASHKENT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441168631, -1247547600, 354909600, 370717200, 386445600, 402253200, 417981600,
			433789200, 449604000, 465336000, 481060800, 496785600, 512510400, 528235200, 543960000,
			559684800, 575409600, 591134400, 606859200, 622584000, 638308800, 654638400, 670363200,
			686091600,
		],
		transition_types: &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 16631, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+05", "+07", "+06", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_TBILISI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840151551, -1441162751, -405140400, 354916800, 370724400, 386452800, 402260400,
			417988800, 433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400,
			543967200, 559692000, 575416800, 591141600, 606866400, 622591200, 638316000, 654645600,
			670370400, 686098800, 701816400, 717537600, 733266000, 748987200, 764715600, 780436800,
			796161600, 811882800, 828216000, 859662000, 877806000, 891115200, 909255600, 922564800,
			941310000, 954014400, 972759600, 985464000, 1004209200, 1017518400, 1035658800,
			1048968000, 1067108400, 1080417600, 1088276400, 1099177200, 1111878000,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 5, 2, 5, 2, 5, 2,
			5, 4, 3, 4, 3, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 5, 2, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 10751, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10751, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 17 },
		],
		time_zone_designations: &["LMT", "TBMT", "+03", "+05", "+04", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const ASIA_THIMBU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-706341516, 560025000],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 21516, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 19800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 10 },
		],
		time_zone_designations: &["LMT", "+0530", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ASIA_TOMSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1578807591, -1247551200, 354906000, 370713600, 386442000, 402249600, 417978000,
			433785600, 449600400, 465332400, 481057200, 496782000, 512506800, 528231600, 543956400,
			559681200, 575406000, 591130800, 606855600, 622580400, 638305200, 654634800, 670359600,
			686088000, 695764800, 701809200, 717534000, 733258800, 748983600, 764708400, 780433200,
			796158000, 811882800, 828212400, 846356400, 859662000, 877806000, 891111600, 909255600,
			922561200, 941310000, 954010800, 972759600, 985460400, 1004209200, 1017514800,
			1020193200, 1035662400, 1048968000, 1067112000, 1080417600, 1099166400, 1111867200,
			1130616000, 1143316800, 1162065600, 1174766400, 1193515200, 1206820800, 1224964800,
			1238270400, 1256414400, 1269720000, 1288468800, 1301169600, 1414263600, 1464465600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 3, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 20391, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+06", "+08", "+07", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_UJUNG_PANDANG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1577951856, -1172908656, -880272000, -766054800],
		transition_types: &[1, 2, 3, 4],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 28656, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28656, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "MMT", "+08", "+09", "WITA"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WITA", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_ULAN_BATOR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2032931252, 252435600, 417974400, 433782000, 449596800, 465318000, 481046400,
			496767600, 512496000, 528217200, 543945600, 559666800, 575395200, 591116400, 606844800,
			622566000, 638294400, 654620400, 670348800, 686070000, 701798400, 717519600, 733248000,
			748969200, 764697600, 780418800, 796147200, 811868400, 828201600, 843922800, 859651200,
			875372400, 891100800, 906822000, 988394400, 1001696400, 1017424800, 1033146000,
			1048874400, 1064595600, 1080324000, 1096045200, 1111773600, 1127494800, 1143223200,
			1159549200, 1427479200, 1443193200, 1458928800, 1474642800,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 25652, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "+07", "+09", "+08"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+08", offset: -28800 },
		dst_info: None,
	},
};
const ASIA_UST_NERA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1579426374, -1247558400, 354898800, 370699200, 386427600, 402235200, 417963600,
			433771200, 449586000, 465318000, 481042800, 496767600, 512492400, 528217200, 543942000,
			559666800, 575391600, 591116400, 606841200, 622566000, 638290800, 654620400, 670345200,
			686073600, 695750400, 701794800, 717519600, 733244400, 748969200, 764694000, 780418800,
			796143600, 811868400, 828198000, 846342000, 859647600, 877791600, 891097200, 909241200,
			922546800, 941295600, 953996400, 972745200, 985446000, 1004194800, 1017500400,
			1035644400, 1048950000, 1067094000, 1080399600, 1099148400, 1111849200, 1130598000,
			1143298800, 1162047600, 1174748400, 1193497200, 1206802800, 1224946800, 1238252400,
			1256396400, 1269702000, 1288450800, 1301151600, 1315828800, 1414249200,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 5, 6, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 7, 3, 6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 34374, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+08", "+09", "+11", "+12", "+11", "+10", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+10", offset: -36000 },
		dst_info: None,
	},
};
const ASIA_VIENTIANE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2840164924, -1570084924],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 24124, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 24124, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "BMT", "+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ASIA_VLADIVOSTOK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1487321251, -1247562000, 354895200, 370702800, 386431200, 402238800, 417967200,
			433774800, 449589600, 465321600, 481046400, 496771200, 512496000, 528220800, 543945600,
			559670400, 575395200, 591120000, 606844800, 622569600, 638294400, 654624000, 670348800,
			686077200, 695754000, 701798400, 717523200, 733248000, 748972800, 764697600, 780422400,
			796147200, 811872000, 828201600, 846345600, 859651200, 877795200, 891100800, 909244800,
			922550400, 941299200, 954000000, 972748800, 985449600, 1004198400, 1017504000,
			1035648000, 1048953600, 1067097600, 1080403200, 1099152000, 1111852800, 1130601600,
			1143302400, 1162051200, 1174752000, 1193500800, 1206806400, 1224950400, 1238256000,
			1256400000, 1269705600, 1288454400, 1301155200, 1414249200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 31651, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+09", "+11", "+10", "+10", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+10", offset: -36000 },
		dst_info: None,
	},
};
const ASIA_YAKUTSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1579423138, -1247558400, 354898800, 370706400, 386434800, 402242400, 417970800,
			433778400, 449593200, 465325200, 481050000, 496774800, 512499600, 528224400, 543949200,
			559674000, 575398800, 591123600, 606848400, 622573200, 638298000, 654627600, 670352400,
			686080800, 695757600, 701802000, 717526800, 733251600, 748976400, 764701200, 780426000,
			796150800, 811875600, 828205200, 846349200, 859654800, 877798800, 891104400, 909248400,
			922554000, 941302800, 954003600, 972752400, 985453200, 1004202000, 1017507600,
			1035651600, 1048957200, 1067101200, 1080406800, 1099155600, 1111856400, 1130605200,
			1143306000, 1162054800, 1174755600, 1193504400, 1206810000, 1224954000, 1238259600,
			1256403600, 1269709200, 1288458000, 1301158800, 1414252800,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 31138, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+08", "+10", "+09", "+09", "+10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+09", offset: -32400 },
		dst_info: None,
	},
};
const ASIA_YEKATERINBURG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1688270553, -1592610305, -1247544000, 354913200, 370720800, 386449200, 402256800,
			417985200, 433792800, 449607600, 465339600, 481064400, 496789200, 512514000, 528238800,
			543963600, 559688400, 575413200, 591138000, 606862800, 622587600, 638312400, 654642000,
			670366800, 686095200, 695772000, 701816400, 717541200, 733266000, 748990800, 764715600,
			780440400, 796165200, 811890000, 828219600, 846363600, 859669200, 877813200, 891118800,
			909262800, 922568400, 941317200, 954018000, 972766800, 985467600, 1004216400,
			1017522000, 1035666000, 1048971600, 1067115600, 1080421200, 1099170000, 1111870800,
			1130619600, 1143320400, 1162069200, 1174770000, 1193518800, 1206824400, 1224968400,
			1238274000, 1256418000, 1269723600, 1288472400, 1301173200, 1414267200,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 5, 2, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 6, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 14553, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 13505, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "PMT", "+04", "+06", "+05", "+05", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ASIA_YEREVAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441162680, -405140400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591141600, 606866400, 622591200, 638316000, 654645600, 670370400,
			686098800, 701823600, 717548400, 733273200, 748998000, 764722800, 780447600, 796172400,
			811897200, 859672800, 877816800, 891122400, 909266400, 922572000, 941320800, 954021600,
			972770400, 985471200, 1004220000, 1017525600, 1035669600, 1048975200, 1067119200,
			1080424800, 1099173600, 1111874400, 1130623200, 1143324000, 1162072800, 1174773600,
			1193522400, 1206828000, 1224972000, 1238277600, 1256421600, 1269727200, 1288476000,
			1301176800, 1319925600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 10680, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+04", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const ATLANTIC_AZORES: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713904240, -1830376800, -1689548400, -1677794400, -1667430000, -1647730800,
			-1635807600, -1616194800, -1604358000, -1584658800, -1572735600, -1553036400,
			-1541199600, -1521500400, -1442444400, -1426806000, -1379286000, -1364770800,
			-1348441200, -1333321200, -1316386800, -1301266800, -1284332400, -1269817200,
			-1221433200, -1206918000, -1191193200, -1175468400, -1127689200, -1111964400,
			-1096844400, -1080514800, -1063580400, -1049065200, -1033340400, -1017615600,
			-1002495600, -986166000, -969231600, -950482800, -942015600, -922662000, -906937200,
			-891126000, -877302000, -873676800, -864000000, -857948400, -845852400, -842832000,
			-831340800, -825894000, -814402800, -810777600, -799891200, -794444400, -782953200,
			-779328000, -768441600, -762994800, -749084400, -733359600, -717624000, -701899200,
			-686174400, -670449600, -654724800, -639000000, -623275200, -607550400, -591825600,
			-575496000, -559771200, -544046400, -528321600, -512596800, -496872000, -481147200,
			-465422400, -449697600, -433972800, -417643200, -401918400, -386193600, -370468800,
			-354744000, -339019200, -323294400, -307569600, -291844800, -276120000, -260395200,
			-244670400, -228340800, -212616000, -196891200, -181166400, -165441600, -149716800,
			-133992000, -118267200, 228272400, 243997200, 260326800, 276051600, 291776400,
			307504800, 323226000, 338954400, 354679200, 370404000, 386128800, 401853600, 417582000,
			433303200, 449028000, 465357600, 481082400, 496807200, 512532000, 528256800, 543981600,
			559706400, 575431200, 591156000, 606880800, 622605600, 638330400, 654660000, 670384800,
			686109600, 701834400, 717559200, 733280400, 749005200, 764730000, 780454800, 796179600,
			811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 2, 3, 2, 4, 2, 3, 2, 4, 2, 3, 2, 4,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 6, 4, 5, 4, 5, 4, 5, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -6160, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -6872, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "HMT", "-01", "-02", "+00", "-01", "WET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-01", offset: 3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "+00", offset: 0 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 0 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 3600 },
		}),
	},
};
const ATLANTIC_BERMUDA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524506042, -1664307642, -1648932042, -1632080442, -1618692042, -1262281242,
			-882727200, -858538800, -845229600, -825879600, -814384800, -793825200, -782935200,
			-762375600, -713988000, -703710000, -681933600, -672865200, -650484000, -641415600,
			-618429600, -609966000, -586980000, -578516400, -555530400, -546462000, -429127200,
			-415825200, 136360800, 152082000, 167810400, 183531600, 199260000, 215586000,
			230709600, 247035600, 262764000, 278485200, 294213600, 309934800, 325663200, 341384400,
			357112800, 372834000, 388562400, 404888400, 420012000, 436338000, 452066400, 467787600,
			483516000, 499237200, 514965600, 530686800, 544600800, 562136400, 576050400, 594190800,
			607500000, 625640400, 638949600, 657090000, 671004000, 688539600, 702453600, 719989200,
			733903200, 752043600, 765352800, 783493200, 796802400, 814942800, 828856800, 846392400,
			860306400, 877842000, 891756000, 909291600, 923205600, 941346000, 954655200, 972795600,
			986104800, 1004245200, 1018159200, 1035694800, 1049608800, 1067144400, 1081058400,
			1099198800, 1112508000, 1130648400, 1143957600, 1162098000, 1173592800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15558, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -11958, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -15558, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "BST", "BMT", "ADT", "AST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ADT", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const ATLANTIC_CANARY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1509663504, -733874400, 323827200, 338950800, 354675600, 370400400, 386125200,
			401850000, 417574800, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400,
			528253200, 543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800,
			654656400, 670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000,
			780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -3696, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "-01", "WET", "WEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WET", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "WEST", offset: -3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const ATLANTIC_CAPE_VERDE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1830376800, -862610400, -764118000, 186120000],
		transition_types: &[1, 2, 1, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -5644, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-01", "-01"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-01", offset: 3600 },
		dst_info: None,
	},
};
const ATLANTIC_FAEROE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1955748776, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600,
			449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800,
			575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000,
			701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400,
			828234000,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -1624, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "WET", "WEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WET", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "WEST", offset: -3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const ATLANTIC_MADEIRA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713906344, -1830380400, -1689552000, -1677798000, -1667433600, -1647734400,
			-1635811200, -1616198400, -1604361600, -1584662400, -1572739200, -1553040000,
			-1541203200, -1521504000, -1442448000, -1426809600, -1379289600, -1364774400,
			-1348444800, -1333324800, -1316390400, -1301270400, -1284336000, -1269820800,
			-1221436800, -1206921600, -1191196800, -1175472000, -1127692800, -1111968000,
			-1096848000, -1080518400, -1063584000, -1049068800, -1033344000, -1017619200,
			-1002499200, -986169600, -969235200, -950486400, -942019200, -922665600, -906940800,
			-891129600, -877305600, -873680400, -864003600, -857952000, -845856000, -842835600,
			-831344400, -825897600, -814406400, -810781200, -799894800, -794448000, -782956800,
			-779331600, -768445200, -762998400, -749088000, -733363200, -717627600, -701902800,
			-686178000, -670453200, -654728400, -639003600, -623278800, -607554000, -591829200,
			-575499600, -559774800, -544050000, -528325200, -512600400, -496875600, -481150800,
			-465426000, -449701200, -433976400, -417646800, -401922000, -386197200, -370472400,
			-354747600, -339022800, -323298000, -307573200, -291848400, -276123600, -260398800,
			-244674000, -228344400, -212619600, -196894800, -181170000, -165445200, -149720400,
			-133995600, -118270800, 228268800, 243993600, 260323200, 276048000, 291772800,
			307501200, 323222400, 338950800, 354675600, 370400400, 386125200, 401850000, 417578400,
			433299600, 449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000,
			559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200,
			686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600,
			811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 2, 3, 2, 4, 2, 3, 2, 4, 2, 3, 2, 4,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -4056, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -4056, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 25 },
		],
		time_zone_designations: &["LMT", "FMT", "+00", "-01", "+01", "WEST", "WET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WET", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "WEST", offset: -3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const ATLANTIC_SOUTH_GEORGIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2524512832],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -8768, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-02", offset: 7200 },
		dst_info: None,
	},
};
const ATLANTIC_STANLEY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524507716, -1824235716, -1018209600, -1003093200, -986760000, -971643600, -954705600,
			-939589200, -923256000, -908139600, -891806400, -876690000, -860356800, -852066000,
			420609600, 433306800, 452052000, 464151600, 483501600, 495601200, 514350000, 527054400,
			545799600, 558504000, 577249200, 589953600, 608698800, 621403200, 640753200, 652852800,
			672202800, 684907200, 703652400, 716356800, 735102000, 747806400, 766551600, 779256000,
			798001200, 810705600, 830055600, 842760000, 861505200, 874209600, 892954800, 905659200,
			924404400, 937108800, 955854000, 968558400, 987310800, 999410400, 1019365200,
			1030860000, 1050814800, 1062914400, 1082264400, 1094364000, 1113714000, 1125813600,
			1145163600, 1157263200, 1176613200, 1188712800, 1208667600, 1220767200, 1240117200,
			1252216800, 1271566800, 1283666400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 5, 4, 5, 4, 5, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -13884, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -13884, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "SMT", "-03", "-04", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const AUSTRALIA_EUCLA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2337928528, -1672555500, -1665384300, -883637100, -876120300, -860395500, -844670700,
			152039700, 162926100, 436295700, 447182100, 690311700, 699383700, 1165079700,
			1174756500, 1193505300, 1206810900, 1224954900, 1238260500,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 30928, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 35100, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 31500, is_dst: false, idx: 10 },
		],
		time_zone_designations: &["LMT", "+0945", "+0845"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+0845", offset: -31500 },
		dst_info: None,
	},
};
const AUSTRALIA_LHI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2364114980, 352216800, 372785400, 384273000, 404839800, 415722600, 436289400,
			447172200, 467739000, 478621800, 499188600, 511282800, 530033400, 542732400, 562087800,
			574786800, 594142200, 606236400, 625591800, 636476400, 657041400, 667926000, 688491000,
			699375600, 719940600, 731430000, 751995000, 762879600, 783444600, 794329200, 814894200,
			828198000, 846343800, 859647600, 877793400, 891097200, 909243000, 922546800, 941297400,
			953996400, 967303800, 985446000, 1004196600, 1017500400, 1035646200, 1048950000,
			1067095800, 1080399600, 1099150200, 1111849200, 1130599800, 1143903600, 1162049400,
			1174748400, 1193499000, 1207407600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 38180, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 41400, is_dst: true, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 37800, is_dst: false, idx: 15 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 21 },
		],
		time_zone_designations: &["LMT", "AEST", "+1130", "+1030", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+1030", offset: -37800 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "+11", offset: -39600 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 7200 },
		}),
	},
};
const AUSTRALIA_LINDEMAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2366790956, -1672560000, -1665388800, -883641600, -876124800, -860400000, -844675200,
			-828345600, -813225600, 57686400, 67968000, 625593600, 636480000, 657043200, 667929600,
			688492800, 699379200, 719942400, 731433600, 751996800, 762883200,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 35756, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AEDT", "AEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AEST", offset: -36000 },
		dst_info: None,
	},
};
const AUSTRALIA_NSW: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2364113092, -1672560000, -1665388800, -883641600, -876124800, -860400000, -844675200,
			-828345600, -813225600, 57686400, 67968000, 89136000, 100022400, 120585600, 131472000,
			152035200, 162921600, 183484800, 194976000, 215539200, 226425600, 246988800, 257875200,
			278438400, 289324800, 309888000, 320774400, 341337600, 352224000, 372787200, 386697600,
			404841600, 415728000, 436291200, 447177600, 467740800, 478627200, 499190400, 511286400,
			530035200, 542736000, 562089600, 574790400, 594144000, 606240000, 625593600, 636480000,
			657043200, 667929600, 688492800, 699379200, 719942400, 731433600, 751996800, 762883200,
			783446400, 794332800, 814896000, 828201600, 846345600, 859651200, 877795200, 891100800,
			909244800, 922550400, 941299200, 954000000, 967305600, 985449600, 1004198400,
			1017504000, 1035648000, 1048953600, 1067097600, 1080403200, 1099152000, 1111852800,
			1130601600, 1143907200, 1162051200, 1174752000, 1193500800, 1207411200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 36292, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AEDT", "AEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AEST", offset: -36000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AEDT", offset: -39600 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const AUSTRALIA_NORTH: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2364108200, -2230189200, -1672558200, -1665387000, -883639800, -876123000, -860398200,
			-844673400, -828343800, -813223800,
		],
		transition_types: &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 31400, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 37800, is_dst: true, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 34200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "ACST", "ACDT", "ACST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "ACST", offset: -34200 },
		dst_info: None,
	},
};
const AUSTRALIA_QUEENSLAND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2366791928, -1672560000, -1665388800, -883641600, -876124800, -860400000, -844675200,
			-828345600, -813225600, 57686400, 67968000, 625593600, 636480000, 657043200, 667929600,
			688492800, 699379200,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 36728, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AEDT", "AEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AEST", offset: -36000 },
		dst_info: None,
	},
};
const AUSTRALIA_SOUTH: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2364110060, -2230189200, -1672558200, -1665387000, -883639800, -876123000, -860398200,
			-844673400, -828343800, -813223800, 57688200, 67969800, 89137800, 100024200, 120587400,
			131473800, 152037000, 162923400, 183486600, 194977800, 215541000, 226427400, 246990600,
			257877000, 278440200, 289326600, 309889800, 320776200, 341339400, 352225800, 372789000,
			384280200, 404843400, 415729800, 436293000, 447179400, 467742600, 478629000, 499192200,
			511288200, 530037000, 542737800, 562091400, 574792200, 594145800, 606241800, 625595400,
			637691400, 657045000, 667931400, 688494600, 701195400, 719944200, 731435400, 751998600,
			764094600, 783448200, 796149000, 814897800, 828203400, 846347400, 859653000, 877797000,
			891102600, 909246600, 922552200, 941301000, 954001800, 972750600, 985451400,
			1004200200, 1017505800, 1035649800, 1048955400, 1067099400, 1080405000, 1099153800,
			1111854600, 1130603400, 1143909000, 1162053000, 1174753800, 1193502600, 1207413000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 33260, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 37800, is_dst: true, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 34200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "ACST", "ACDT", "ACST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "ACST", offset: -34200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ACDT", offset: -37800 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const AUSTRALIA_TASMANIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2345795356, -1680508800, -1665388800, -1646640000, -1635753600, -1615190400,
			-1604304000, -883641600, -876124800, -860400000, -844675200, -828345600, -813225600,
			-71136000, -55411200, -37267200, -25776000, -5817600, 5673600, 25632000, 37728000,
			57686400, 67968000, 89136000, 100022400, 120585600, 131472000, 152035200, 162921600,
			183484800, 194976000, 215539200, 226425600, 246988800, 257875200, 278438400, 289324800,
			309888000, 320774400, 341337600, 352224000, 372787200, 386092800, 404841600, 417542400,
			436291200, 447177600, 467740800, 478627200, 499190400, 510076800, 530035200, 542736000,
			562089600, 574790400, 594144000, 606240000, 625593600, 637689600, 657043200, 670348800,
			686678400, 701798400, 718128000, 733248000, 749577600, 764697600, 781027200, 796147200,
			812476800, 828201600, 844531200, 859651200, 875980800, 891100800, 907430400, 922550400,
			938880000, 954000000, 967305600, 985449600, 1002384000, 1017504000, 1033833600,
			1048953600, 1065283200, 1080403200, 1096732800, 1111852800, 1128182400, 1143907200,
			1159632000, 1174752000, 1191686400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 35356, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AEDT", "AEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AEST", offset: -36000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AEDT", offset: -39600 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const AUSTRALIA_VICTORIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2364111592, -1672560000, -1665388800, -883641600, -876124800, -860400000, -844675200,
			-828345600, -813225600, 57686400, 67968000, 89136000, 100022400, 120585600, 131472000,
			152035200, 162921600, 183484800, 194976000, 215539200, 226425600, 246988800, 257875200,
			278438400, 289324800, 309888000, 320774400, 341337600, 352224000, 372787200, 384278400,
			404841600, 415728000, 436291200, 447177600, 467740800, 478627200, 499190400, 511286400,
			530035200, 542736000, 561484800, 574790400, 594144000, 606240000, 625593600, 637689600,
			657043200, 667929600, 688492800, 699379200, 719942400, 731433600, 751996800, 762883200,
			783446400, 796147200, 814896000, 828201600, 846345600, 859651200, 877795200, 891100800,
			909244800, 922550400, 941299200, 954000000, 967305600, 985449600, 1004198400,
			1017504000, 1035648000, 1048953600, 1067097600, 1080403200, 1099152000, 1111852800,
			1130601600, 1143907200, 1162051200, 1174752000, 1193500800, 1207411200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 34792, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AEDT", "AEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AEST", offset: -36000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AEDT", offset: -39600 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const AUSTRALIA_WEST: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2337925404, -1672552800, -1665381600, -883634400, -876117600, -860392800, -844668000,
			152042400, 162928800, 436298400, 447184800, 690314400, 699386400, 1165082400,
			1174759200, 1193508000, 1206813600, 1224957600, 1238263200,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 27804, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AWDT", "AWST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AWST", offset: -28800 },
		dst_info: None,
	},
};
const AUSTRALIA_YANCOWINNA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2364110748, -2314951200, -2230189200, -1672558200, -1665387000, -883639800,
			-876123000, -860398200, -844673400, -828343800, -813223800, 57688200, 67969800,
			89137800, 100024200, 120587400, 131473800, 152037000, 162923400, 183486600, 194977800,
			215541000, 226427400, 246990600, 257877000, 278440200, 289326600, 309889800, 320776200,
			341339400, 352225800, 372789000, 386699400, 404843400, 415729800, 436293000, 447179400,
			467742600, 478629000, 499192200, 511288200, 530037000, 542737800, 562091400, 574792200,
			594145800, 606241800, 625595400, 636481800, 657045000, 667931400, 688494600, 699381000,
			719944200, 731435400, 751998600, 762885000, 783448200, 794334600, 814897800, 828203400,
			846347400, 859653000, 877797000, 891102600, 909246600, 922552200, 941301000, 954001800,
			972750600, 985451400, 1004200200, 1017505800, 1035649800, 1048955400, 1067099400,
			1080405000, 1099153800, 1111854600, 1130603400, 1143909000, 1162053000, 1174753800,
			1193502600, 1207413000,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 33948, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 37800, is_dst: true, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 34200, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "AEST", "ACST", "ACDT", "ACST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "ACST", offset: -34200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ACDT", offset: -37800 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const BRAZIL_ACRE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767209328, -1206950400, -1191355200, -1175367600, -1159819200, -633812400,
			-622062000, -602276400, -591825600, -570740400, -560203200, -539118000, -531345600,
			-191358000, -184190400, -155156400, -150062400, -128890800, -121118400, -99946800,
			-89582400, -68410800, -57960000, 499755600, 511243200, 530600400, 540273600, 562136400,
			571204800, 1214283600, 1384056000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16272, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-04", "-05", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-05", offset: 18000 },
		dst_info: None,
	},
};
const BRAZIL_DE_NORONHA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767217820, -1206961200, -1191366000, -1175378400, -1159830000, -633823200,
			-622072800, -602287200, -591836400, -570751200, -560214000, -539128800, -531356400,
			-191368800, -184201200, -155167200, -150073200, -128901600, -121129200, -99957600,
			-89593200, -68421600, -57970800, 499744800, 511232400, 530589600, 540262800, 562125600,
			571194000, 592970400, 602038800, 624420000, 634698000, 938916000, 951613200, 970970400,
			971571600, 1003024800, 1013907600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -7780, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -3600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-01", "-02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-02", offset: 7200 },
		dst_info: None,
	},
};
const BRAZIL_EAST: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767214412, -1206957600, -1191362400, -1175374800, -1159826400, -633819600,
			-622069200, -602283600, -591832800, -570747600, -560210400, -539125200, -531352800,
			-195426000, -184197600, -155163600, -150069600, -128898000, -121125600, -99954000,
			-89589600, -68418000, -57967200, 499748400, 511236000, 530593200, 540266400, 562129200,
			571197600, 592974000, 602042400, 624423600, 634701600, 656478000, 666756000, 687927600,
			697600800, 719982000, 728445600, 750826800, 761709600, 782276400, 793159200, 813726000,
			824004000, 844570800, 856058400, 876106800, 888717600, 908074800, 919562400, 938919600,
			951616800, 970974000, 982461600, 1003028400, 1013911200, 1036292400, 1045360800,
			1066532400, 1076810400, 1099364400, 1108864800, 1129431600, 1140314400, 1162695600,
			1172368800, 1192330800, 1203213600, 1224385200, 1234663200, 1255834800, 1266717600,
			1287284400, 1298167200, 1318734000, 1330221600, 1350788400, 1361066400, 1382238000,
			1392516000, 1413687600, 1424570400, 1445137200, 1456020000, 1476586800, 1487469600,
			1508036400, 1518919200, 1541300400, 1550368800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -11188, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-02", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const BRAZIL_WEST: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767211196, -1206954000, -1191358800, -1175371200, -1159822800, -633816000,
			-622065600, -602280000, -591829200, -570744000, -560206800, -539121600, -531349200,
			-191361600, -184194000, -155160000, -150066000, -128894400, -121122000, -99950400,
			-89586000, -68414400, -57963600, 499752000, 511239600, 530596800, 540270000, 562132800,
			571201200, 750830400, 761713200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -14404, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-03", "-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const CET: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1693706400, -1680483600, -1663455600, -1650150000, -1632006000, -1618700400,
			-938905200, -857257200, -844556400, -828226800, -812502000, -796777200, -781052400,
			-766623600, 228877200, 243997200, 260326800, 276051600, 291776400, 307501200,
			323830800, 338950800, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600,
			449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800,
			575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000,
			701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400,
			828234000,
		],
		transition_types: &[
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 5 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 0 },
		],
		time_zone_designations: &["CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const CST_6_CDT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1633276800, -1615136400, -1601827200, -1583686800, -880214400, -769395600, -765392400,
			-84384000, -68662800, -52934400, -37213200, -21484800, -5763600, 9964800, 25686000,
			41414400, 57740400, 73468800, 89190000, 104918400, 120639600, 126691200, 152089200,
			162374400, 183538800, 199267200, 215593200, 230716800, 247042800, 262771200, 278492400,
			294220800, 309942000, 325670400, 341391600, 357120000, 372841200, 388569600, 404895600,
			420019200, 436345200, 452073600, 467794800, 483523200, 499244400, 514972800, 530694000,
			544608000, 562143600, 576057600, 594198000, 607507200, 625647600, 638956800, 657097200,
			671011200, 688546800, 702460800, 719996400, 733910400, 752050800, 765360000, 783500400,
			796809600, 814950000, 828864000, 846399600, 860313600, 877849200, 891763200, 909298800,
			923212800, 941353200, 954662400, 972802800, 986112000, 1004252400, 1018166400,
			1035702000, 1049616000, 1067151600, 1081065600, 1099206000, 1112515200, 1130655600,
			1143964800, 1162105200, 1173600000,
		],
		transition_types: &[
			1, 0, 1, 0, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["CST", "CDT", "CWT", "CPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_ATLANTIC: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2131645536, -1696276800, -1680469200, -1632074400, -1615143600, -1566763200,
			-1557090000, -1535486400, -1524949200, -1504468800, -1493413200, -1472414400,
			-1461963600, -1440964800, -1429390800, -1409515200, -1396731600, -1376856000,
			-1366491600, -1346616000, -1333832400, -1313956800, -1303678800, -1282507200,
			-1272661200, -1251057600, -1240088400, -1219608000, -1207429200, -1188763200,
			-1175979600, -1157313600, -1143925200, -1124049600, -1113771600, -1091390400,
			-1081026000, -1059854400, -1050786000, -1030910400, -1018126800, -999460800,
			-986677200, -965592000, -955227600, -935956800, -923173200, -904507200, -891723600,
			-880221600, -769395600, -765399600, -747252000, -733950000, -715802400, -702500400,
			-684352800, -671050800, -652903200, -639601200, -589399200, -576097200, -557949600,
			-544647600, -526500000, -513198000, -495050400, -481748400, -431546400, -418244400,
			-400096800, -386794800, -368647200, -355345200, -337197600, -323895600, -242244000,
			-226522800, -210794400, -195073200, -179344800, -163623600, -147895200, -131569200,
			-116445600, -100119600, -84391200, -68670000, -52941600, -37220400, -21492000,
			-5770800, 9957600, 25678800, 41407200, 57733200, 73461600, 89182800, 104911200,
			120632400, 136360800, 152082000, 167810400, 183531600, 199260000, 215586000, 230709600,
			247035600, 262764000, 278485200, 294213600, 309934800, 325663200, 341384400, 357112800,
			372834000, 388562400, 404888400, 420012000, 436338000, 452066400, 467787600, 483516000,
			499237200, 514965600, 530686800, 544600800, 562136400, 576050400, 594190800, 607500000,
			625640400, 638949600, 657090000, 671004000, 688539600, 702453600, 719989200, 733903200,
			752043600, 765352800, 783493200, 796802400, 814942800, 828856800, 846392400, 860306400,
			877842000, 891756000, 909291600, 923205600, 941346000, 954655200, 972795600, 986104800,
			1004245200, 1018159200, 1035694800, 1049608800, 1067144400, 1081058400, 1099198800,
			1112508000, 1130648400, 1143957600, 1162098000, 1173592800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -15264, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "ADT", "AST", "AWT", "APT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AST", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "ADT", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_CENTRAL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2602258284, -1694368800, -1681671600, -1632067200, -1615136400, -1029686400,
			-1018198800, -880214400, -769395600, -765392400, -746035200, -732733200, -715795200,
			-702493200, -684345600, -671043600, -652896000, -639594000, -620755200, -607626000,
			-589392000, -576090000, -557942400, -544640400, -526492800, -513190800, -495043200,
			-481741200, -463593600, -450291600, -431539200, -418237200, -400089600, -386787600,
			-368640000, -355338000, -337190400, -321469200, -305740800, -292438800, -210787200,
			-198090000, -116438400, -100108800, -84384000, -68659200, -52934400, -37209600,
			-21484800, -5760000, 9964800, 25689600, 41414400, 57744000, 73468800, 89193600,
			104918400, 120643200, 136368000, 152092800, 167817600, 183542400, 199267200, 215596800,
			230716800, 247046400, 262771200, 278496000, 294220800, 309945600, 325670400, 341395200,
			357120000, 372844800, 388569600, 404899200, 420019200, 436348800, 452073600, 467798400,
			483523200, 499248000, 514972800, 530697600, 544608000, 562147200, 576057600, 594201600,
			607507200, 625651200, 638956800, 657100800, 671011200, 688550400, 702460800, 720000000,
			733910400, 752054400, 765360000, 783504000, 796809600, 814953600, 828864000, 846403200,
			860313600, 877852800, 891763200, 909302400, 923212800, 941356800, 954662400, 972806400,
			986112000, 1004256000, 1018166400, 1035705600, 1049616000, 1067155200, 1081065600,
			1099209600, 1112515200, 1130659200, 1143964800, 1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -23316, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_EASTERN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2366736148, -1632070800, -1615140000, -1601753400, -1583697600, -1567357200,
			-1554667200, -1534698000, -1524074400, -1503248400, -1492365600, -1471798800,
			-1460916000, -1440954000, -1428861600, -1409504400, -1397412000, -1378054800,
			-1365962400, -1346605200, -1333908000, -1315155600, -1301853600, -1283706000,
			-1270404000, -1252256400, -1238954400, -1220806800, -1207504800, -1188752400,
			-1176055200, -1157302800, -1144000800, -1125853200, -1112551200, -1094403600,
			-1081101600, -1062954000, -1049652000, -1031504400, -1018202400, -1000054800,
			-986752800, -968000400, -955303200, -936550800, -880218000, -769395600, -765396000,
			-747248400, -733946400, -715806000, -702504000, -684356400, -671054400, -652906800,
			-634161600, -620845200, -602704800, -589395600, -576093600, -557946000, -544644000,
			-526496400, -513194400, -495046800, -481744800, -463597200, -450295200, -431542800,
			-418240800, -400093200, -384372000, -368643600, -352922400, -337194000, -321472800,
			-305744400, -289418400, -273690000, -257968800, -242240400, -226519200, -210790800,
			-195069600, -179341200, -163620000, -147891600, -131565600, -116442000, -100116000,
			-84387600, -68666400, -52938000, -37216800, -21488400, -5767200, 9961200, 25682400,
			41410800, 57736800, 73465200, 89186400, 104914800, 120636000, 136364400, 152085600,
			167814000, 183535200, 199263600, 215589600, 230713200, 247039200, 262767600, 278488800,
			294217200, 309938400, 325666800, 341388000, 357116400, 372837600, 388566000, 404892000,
			420015600, 436341600, 452070000, 467791200, 483519600, 499240800, 514969200, 530690400,
			544604400, 562140000, 576054000, 594194400, 607503600, 625644000, 638953200, 657093600,
			671007600, 688543200, 702457200, 719992800, 733906800, 752047200, 765356400, 783496800,
			796806000, 814946400, 828860400, 846396000, 860310000, 877845600, 891759600, 909295200,
			923209200, 941349600, 954658800, 972799200, 986108400, 1004248800, 1018162800,
			1035698400, 1049612400, 1067148000, 1081062000, 1099202400, 1112511600, 1130652000,
			1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -19052, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "EDT", "EST", "EWT", "EPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_MOUNTAIN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1998663968, -1632063600, -1615132800, -1600614000, -1596816000, -1567954800,
			-1551628800, -1536505200, -1523203200, -1504450800, -1491753600, -1473001200,
			-1459699200, -880210800, -769395600, -765388800, -715791600, -702489600, 73472400,
			89193600, 104922000, 120643200, 136371600, 152092800, 167821200, 183542400, 199270800,
			215596800, 230720400, 247046400, 262774800, 278496000, 294224400, 309945600, 325674000,
			341395200, 357123600, 372844800, 388573200, 404899200, 420022800, 436348800, 452077200,
			467798400, 483526800, 499248000, 514976400, 530697600, 544611600, 562147200, 576061200,
			594201600, 607510800, 625651200, 638960400, 657100800, 671014800, 688550400, 702464400,
			720000000, 733914000, 752054400, 765363600, 783504000, 796813200, 814953600, 828867600,
			846403200, 860317200, 877852800, 891766800, 909302400, 923216400, 941356800, 954666000,
			972806400, 986115600, 1004256000, 1018170000, 1035705600, 1049619600, 1067155200,
			1081069200, 1099209600, 1112518800, 1130659200, 1143968400, 1162108800, 1173603600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -27232, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_NEWFOUNDLAND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713897748, -1664130548, -1650137348, -1632076148, -1615145348, -1598650148,
			-1590100148, -1567286948, -1551565748, -1535837348, -1520116148, -1503782948,
			-1488666548, -1472333348, -1457216948, -1440883748, -1425767348, -1409434148,
			-1394317748, -1377984548, -1362263348, -1346534948, -1330813748, -1314480548,
			-1299364148, -1283030948, -1267914548, -1251581348, -1236464948, -1220131748,
			-1205015348, -1188682148, -1172960948, -1156627748, -1141511348, -1125178148,
			-1110061748, -1096921748, -1093728600, -1078612200, -1061670600, -1048973400,
			-1030221000, -1017523800, -998771400, -986074200, -966717000, -954624600, -935267400,
			-922570200, -903817800, -891120600, -872368200, -769395600, -765401400, -746044200,
			-733347000, -714594600, -701897400, -683145000, -670447800, -651695400, -638998200,
			-619641000, -606943800, -589401000, -576099000, -557951400, -544649400, -526501800,
			-513199800, -495052200, -481750200, -463602600, -450300600, -431548200, -418246200,
			-400098600, -386796600, -368649000, -355347000, -337199400, -323897400, -305749800,
			-289423800, -273695400, -257974200, -242245800, -226524600, -210796200, -195075000,
			-179346600, -163625400, -147897000, -131571000, -116447400, -100121400, -84393000,
			-68671800, -52943400, -37222200, -21493800, -5772600, 9955800, 25677000, 41405400,
			57731400, 73459800, 89181000, 104909400, 120630600, 136359000, 152080200, 167808600,
			183529800, 199258200, 215584200, 230707800, 247033800, 262762200, 278483400, 294211800,
			309933000, 325661400, 341382600, 357111000, 372832200, 388560600, 404886600, 420010200,
			436336200, 452064600, 467785800, 483514200, 499235400, 514963800, 530685000, 544591860,
			562127460, 576041460, 594178260, 607491060, 625631460, 638940660, 657081060, 670995060,
			688530660, 702444660, 719980260, 733894260, 752034660, 765343860, 783484260, 796793460,
			814933860, 828847860, 846383460, 860297460, 877833060, 891747060, 909282660, 923196660,
			941337060, 954646260, 972786660, 986095860, 1004236260, 1018150260, 1035685860,
			1049599860, 1067135460, 1081049460, 1099189860, 1112499060, 1130639460, 1143948660,
			1162089060, 1173583860, 1194143460, 1205033460, 1225593060, 1236483060, 1257042660,
			1268537460, 1289097060, 1299987060, 1320553800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 6, 5, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 7, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -12652, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -9052, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -12652, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -12600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -9000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -5400, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "NDT", "NST", "NDT", "NST", "NPT", "NWT", "NDDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "NST", offset: 12600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "NDT", offset: 9000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_PACIFIC: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713880852, -1632060000, -1615129200, -880207200, -769395600, -765385200, -747237600,
			-733935600, -715788000, -702486000, -684338400, -671036400, -652888800, -639586800,
			-620834400, -608137200, -589384800, -576082800, -557935200, -544633200, -526485600,
			-513183600, -495036000, -481734000, -463586400, -450284400, -431532000, -418230000,
			-400082400, -386780400, -368632800, -355330800, -337183200, -323881200, -305733600,
			-292431600, -273679200, -260982000, -242229600, -226508400, -210780000, -195058800,
			-179330400, -163609200, -147880800, -131554800, -116431200, -100105200, -84376800,
			-68655600, -52927200, -37206000, -21477600, -5756400, 9972000, 25693200, 41421600,
			57747600, 73476000, 89197200, 104925600, 120646800, 136375200, 152096400, 167824800,
			183546000, 199274400, 215600400, 230724000, 247050000, 262778400, 278499600, 294228000,
			309949200, 325677600, 341398800, 357127200, 372848400, 388576800, 404902800, 420026400,
			436352400, 452080800, 467802000, 483530400, 499251600, 514980000, 530701200, 544615200,
			562150800, 576064800, 594205200, 607514400, 625654800, 638964000, 657104400, 671018400,
			688554000, 702468000, 720003600, 733917600, 752058000, 765367200, 783507600, 796816800,
			814957200, 828871200, 846406800, 860320800, 877856400, 891770400, 909306000, 923220000,
			941360400, 954669600, 972810000, 986119200, 1004259600, 1018173600, 1035709200,
			1049623200, 1067158800, 1081072800, 1099213200, 1112522400, 1130662800, 1143972000,
			1162112400, 1173607200,
		],
		transition_types: &[
			2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -29548, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "PDT", "PST", "PWT", "PPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "PST", offset: 28800 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "PDT", offset: 25200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const CANADA_SASKATCHEWAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2030202084, -1632063600, -1615132800, -1251651600, -1238349600, -1220202000,
			-1206900000, -1188752400, -1175450400, -1156698000, -1144000800, -1125248400,
			-1111946400, -1032714000, -1016992800, -1001264400, -986148000, -969814800, -954093600,
			-937760400, -922039200, -906310800, -890589600, -880210800, -769395600, -765388800,
			-748450800, -732729600, -715791600, -702489600, -684342000, -671040000, -652892400,
			-639590400, -620838000, -608140800, -589388400, -576086400, -557938800, -544636800,
			-526489200, -513187200, -495039600, -481737600, -463590000, -450288000, -431535600,
			-418233600, -400086000, -386784000, -337186800, -321465600, -305737200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25116, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const CANADA_YUKON: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2188997988, -1632056400, -1615125600, -1596978000, -1583164800, -880203600,
			-769395600, -765381600, -147884400, -131554800, -121273200, 325677600, 341398800,
			357127200, 372848400, 388576800, 404902800, 420026400, 436352400, 452080800, 467802000,
			483530400, 499251600, 514980000, 530701200, 544615200, 562150800, 576064800, 594205200,
			607514400, 625654800, 638964000, 657104400, 671018400, 688554000, 702468000, 720003600,
			733917600, 752058000, 765367200, 783507600, 796816800, 814957200, 828871200, 846406800,
			860320800, 877856400, 891770400, 909306000, 923220000, 941360400, 954669600, 972810000,
			986119200, 1004259600, 1018173600, 1035709200, 1049623200, 1067158800, 1081072800,
			1099213200, 1112522400, 1130662800, 1143972000, 1162112400, 1173607200, 1194166800,
			1205056800, 1225616400, 1236506400, 1257066000, 1268560800, 1289120400, 1300010400,
			1320570000, 1331460000, 1352019600, 1362909600, 1383469200, 1394359200, 1414918800,
			1425808800, 1446368400, 1457863200, 1478422800, 1489312800, 1509872400, 1520762400,
			1541322000, 1552212000, 1572771600, 1583661600, 1604214000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 5, 2, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6,
			7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7,
			6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6,
			7, 6, 7, 6, 7, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -32412, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 25 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 33 },
		],
		time_zone_designations: &["LMT", "YDT", "YST", "YWT", "YPT", "YDDT", "PST", "PDT", "MST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const CHILE_CONTINENTAL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524504635, -1892661435, -1688410800, -1619205435, -1593806400, -1335986235,
			-1317585600, -1304362800, -1286049600, -1272826800, -1254513600, -1241290800,
			-1222977600, -1209754800, -1191355200, -1178132400, -870552000, -865278000, -740520000,
			-736635600, -718056000, -713649600, -36619200, -23922000, -3355200, 7527600, 24465600,
			37767600, 55915200, 69217200, 87969600, 100666800, 118209600, 132116400, 150868800,
			163566000, 182318400, 195620400, 213768000, 227070000, 245217600, 258519600, 277272000,
			289969200, 308721600, 321418800, 340171200, 353473200, 371620800, 384922800, 403070400,
			416372400, 434520000, 447822000, 466574400, 479271600, 498024000, 510721200, 529473600,
			545194800, 560923200, 574225200, 592372800, 605674800, 624427200, 637124400, 653457600,
			668574000, 687326400, 700628400, 718776000, 732078000, 750225600, 763527600, 781675200,
			794977200, 813729600, 826426800, 845179200, 859690800, 876628800, 889930800, 906868800,
			923194800, 939528000, 952830000, 971582400, 984279600, 1003032000, 1015729200,
			1034481600, 1047178800, 1065931200, 1079233200, 1097380800, 1110682800, 1128830400,
			1142132400, 1160884800, 1173582000, 1192334400, 1206846000, 1223784000, 1237086000,
			1255233600, 1270350000, 1286683200, 1304823600, 1313899200, 1335668400, 1346558400,
			1367118000, 1378612800, 1398567600, 1410062400, 1463281200, 1471147200, 1494730800,
			1502596800, 1526180400, 1534046400, 1554606000, 1567915200, 1586055600, 1599364800,
			1617505200, 1630814400, 1648954800, 1662868800, 1680404400,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 3, 2, 3, 5, 4, 2, 3, 5, 3, 5, 3, 5, 3, 5,
			3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3,
			5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5,
			3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3,
			5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -16965, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -16965, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -10800, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "SMT", "-05", "-04", "-04", "-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "-03", offset: 10800 },
			start_date: TransitionDate { day: MonthWeekDay(9, 1, 6), time: 86400 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 6), time: 86400 },
		}),
	},
};
const CHILE_EASTER_ISLAND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524495352, -1178124152, -36619200, -23922000, -3355200, 7527600, 24465600, 37767600,
			55915200, 69217200, 87969600, 100666800, 118209600, 132116400, 150868800, 163566000,
			182318400, 195620400, 213768000, 227070000, 245217600, 258519600, 277272000, 289969200,
			308721600, 321418800, 340171200, 353473200, 371620800, 384922800, 403070400, 416372400,
			434520000, 447822000, 466574400, 479271600, 498024000, 510721200, 529473600, 545194800,
			560923200, 574225200, 592372800, 605674800, 624427200, 637124400, 653457600, 668574000,
			687326400, 700628400, 718776000, 732078000, 750225600, 763527600, 781675200, 794977200,
			813729600, 826426800, 845179200, 859690800, 876628800, 889930800, 906868800, 923194800,
			939528000, 952830000, 971582400, 984279600, 1003032000, 1015729200, 1034481600,
			1047178800, 1065931200, 1079233200, 1097380800, 1110682800, 1128830400, 1142132400,
			1160884800, 1173582000, 1192334400, 1206846000, 1223784000, 1237086000, 1255233600,
			1270350000, 1286683200, 1304823600, 1313899200, 1335668400, 1346558400, 1367118000,
			1378612800, 1398567600, 1410062400, 1463281200, 1471147200, 1494730800, 1502596800,
			1526180400, 1534046400, 1554606000, 1567915200, 1586055600, 1599364800, 1617505200,
			1630814400, 1648954800, 1662868800, 1680404400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -26248, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -26248, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "EMT", "-06", "-07", "-06", "-05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-06", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "-05", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(9, 1, 6), time: 79200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 6), time: 79200 },
		}),
	},
};
const CUBA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524501832, -1402813824, -1311534000, -1300996800, -933534000, -925675200, -902084400,
			-893620800, -870030000, -862171200, -775681200, -767822400, -744231600, -736372800,
			-144702000, -134251200, -113425200, -102542400, -86295600, -72907200, -54154800,
			-41457600, -21495600, -5774400, 9954000, 25675200, 41403600, 57729600, 73458000,
			87364800, 104907600, 118900800, 136357200, 150436800, 167806800, 183528000, 199256400,
			215582400, 230706000, 247032000, 263365200, 276667200, 290581200, 308721600, 322030800,
			340171200, 358318800, 371620800, 389768400, 403070400, 421218000, 434520000, 452667600,
			466574400, 484117200, 498024000, 511333200, 529473600, 542782800, 560923200, 574837200,
			592372800, 606286800, 623822400, 638946000, 655876800, 671000400, 687330000, 702450000,
			718779600, 733899600, 750229200, 765349200, 781678800, 796798800, 813128400, 828853200,
			844578000, 860302800, 876632400, 891147600, 909291600, 922597200, 941346000, 954651600,
			972795600, 986101200, 1004245200, 1018155600, 1035694800, 1049605200, 1067144400,
			1080450000, 1162098000, 1173589200, 1193547600, 1205643600, 1224997200, 1236488400,
			1256446800, 1268542800, 1288501200, 1300597200, 1321160400, 1333256400, 1352005200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -19768, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -19776, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "HMT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 0 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 3600 },
		}),
	},
};
const EET: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			228877200, 243997200, 260326800, 276051600, 291776400, 307501200, 323830800, 338950800,
			354675600, 370400400, 386125200, 401850000, 417574800, 433299600, 449024400, 465354000,
			481078800, 496803600, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400,
			606877200, 622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600,
			733280400, 749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 5 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 0 },
		],
		time_zone_designations: &["EET", "EEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EST: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -18000,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: None,
	},
};
const EST_5_EDT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1633280400, -1615140000, -1601830800, -1583690400, -880218000, -769395600, -765396000,
			-84387600, -68666400, -52938000, -37216800, -21488400, -5767200, 9961200, 25682400,
			41410800, 57736800, 73465200, 89186400, 104914800, 120636000, 126687600, 152085600,
			162370800, 183535200, 199263600, 215589600, 230713200, 247039200, 262767600, 278488800,
			294217200, 309938400, 325666800, 341388000, 357116400, 372837600, 388566000, 404892000,
			420015600, 436341600, 452070000, 467791200, 483519600, 499240800, 514969200, 530690400,
			544604400, 562140000, 576054000, 594194400, 607503600, 625644000, 638953200, 657093600,
			671007600, 688543200, 702457200, 719992800, 733906800, 752047200, 765356400, 783496800,
			796806000, 814946400, 828860400, 846396000, 860310000, 877845600, 891759600, 909295200,
			923209200, 941349600, 954658800, 972799200, 986108400, 1004248800, 1018162800,
			1035698400, 1049612400, 1067148000, 1081062000, 1099202400, 1112511600, 1130652000,
			1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			1, 0, 1, 0, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["EST", "EDT", "EWT", "EPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const EGYPT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2185409109, -929844000, -923108400, -906170400, -892868400, -875844000, -857790000,
			-844308000, -825822000, -812685600, -794199600, -779853600, -762663600, -399088800,
			-386650800, -368330400, -355114800, -336790800, -323654400, -305168400, -292032000,
			-273632400, -260496000, -242096400, -228960000, -210560400, -197424000, -178938000,
			-165801600, -147402000, -134265600, -115866000, -102643200, -84330000, -71107200,
			-52707600, -39484800, -21171600, -7948800, 10364400, 23587200, 41900400, 55123200,
			73522800, 86745600, 105058800, 118281600, 136594800, 149817600, 168130800, 181353600,
			199753200, 212976000, 231289200, 244512000, 262825200, 276048000, 294361200, 307584000,
			325983600, 339206400, 357519600, 370742400, 396399600, 402278400, 426812400, 433814400,
			452214000, 465436800, 483750000, 496972800, 515286000, 528508800, 546822000, 560044800,
			578444400, 591667200, 610412400, 623203200, 641516400, 654739200, 673052400, 686275200,
			704674800, 717897600, 736210800, 749433600, 767746800, 780969600, 799020000, 812322000,
			830469600, 843771600, 861919200, 875221200, 893368800, 906670800, 925423200, 938725200,
			956872800, 970174800, 988322400, 1001624400, 1019772000, 1033074000, 1051221600,
			1064523600, 1083276000, 1096578000, 1114725600, 1128027600, 1146175200, 1158872400,
			1177624800, 1189112400, 1209074400, 1219957200, 1240524000, 1250802000, 1272578400,
			1281474000, 1284069600, 1285880400, 1400191200, 1403816400, 1406844000, 1411678800,
			1682632800,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 7509, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "EEST", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(4, 5, 5), time: 0 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 4), time: 86400 },
		}),
	},
};
const EIRE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2821649679, -1691962479, -1680471279, -1664143200, -1650146400, -1633903200,
			-1617487200, -1601848800, -1586037600, -1570399200, -1552168800, -1538344800,
			-1522533600, -1507500000, -1490565600, -1473631200, -1460930400, -1442786400,
			-1428876000, -1410732000, -1396216800, -1379282400, -1364767200, -1348437600,
			-1333317600, -1315778400, -1301263200, -1284328800, -1269813600, -1253484000,
			-1238364000, -1221429600, -1206914400, -1189980000, -1175464800, -1159135200,
			-1143410400, -1126476000, -1111960800, -1095631200, -1080511200, -1063576800,
			-1049061600, -1032127200, -1017612000, -1001282400, -986162400, -969228000, -950479200,
			-942012000, -733356000, -719445600, -699487200, -684972000, -668037600, -654732000,
			-636588000, -622072800, -605743200, -590623200, -574293600, -558568800, -542239200,
			-527119200, -512604000, -496274400, -481154400, -464220000, -449704800, -432165600,
			-417650400, -401320800, -386200800, -369266400, -354751200, -337816800, -323301600,
			-306972000, -291852000, -276732000, -257983200, -245282400, -226533600, -213228000,
			-195084000, -182383200, -163634400, -150933600, -132184800, -119484000, -100735200,
			-88034400, -68680800, -59004000, -37242000, 57722400, 69818400, 89172000, 101268000,
			120621600, 132717600, 152071200, 164167200, 183520800, 196221600, 214970400, 227671200,
			246420000, 259120800, 278474400, 290570400, 309924000, 322020000, 341373600, 354675600,
			372819600, 386125200, 404269200, 417574800, 435718800, 449024400, 467773200, 481078800,
			499222800, 512528400, 530672400, 543978000, 562122000, 575427600, 593571600, 606877200,
			625626000, 638326800, 657075600, 670381200, 688525200, 701830800, 719974800, 733280400,
			751424400, 764730000, 782874000, 796179600, 814323600, 828234000,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6,
			7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7, 6, 7,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -1521, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -1521, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 2079, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "DMT", "IST", "BST", "GMT", "IST", "GMT", "IST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "IST", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "GMT", offset: 0 },
			start_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
		}),
	},
};
const ETC_GMT_PLUS_1: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -3600,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-01"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-01", offset: 3600 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_10: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -36000,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-10", offset: 36000 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_11: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -39600,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-11", offset: 39600 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_12: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -43200,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-12", offset: 43200 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_2: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -7200,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-02", offset: 7200 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_3: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -10800,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-03", offset: 10800 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_4: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -14400,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-04", offset: 14400 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_5: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -18000,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-05", offset: 18000 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_6: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -21600,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-06", offset: 21600 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_7: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -25200,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-07", offset: 25200 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_8: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -28800,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-08"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-08", offset: 28800 },
		dst_info: None,
	},
};
const ETC_GMT_PLUS_9: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -32400,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["-09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-09", offset: 32400 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_1: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 0 }],
		time_zone_designations: &["+01"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+01", offset: -3600 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_10: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 36000,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+10", offset: -36000 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_11: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 39600,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_12: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 43200,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_13: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 46800,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+13"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+13", offset: -46800 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_14: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 50400,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+14"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+14", offset: -50400 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_2: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 0 }],
		time_zone_designations: &["+02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+02", offset: -7200 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_3: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 10800,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_4: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 14400,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_5: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 18000,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_6: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 21600,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_7: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 25200,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+07"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+07", offset: -25200 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_8: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 28800,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+08"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+08", offset: -28800 },
		dst_info: None,
	},
};
const ETC_GMT_MINUS_9: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: 32400,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+09", offset: -32400 },
		dst_info: None,
	},
};
const EUROPE_ANDORRA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2177453164, -733881600, 481078800, 496803600, 512528400, 528253200, 543978000,
			559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200,
			686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600,
			811904400, 828234000,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 364, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "WET", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_ASTRAKHAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1441249932, -1247540400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591141600, 606866400, 622594800, 638319600, 654649200, 670374000,
			701820000, 717548400, 733273200, 748998000, 764722800, 780447600, 796172400, 811897200,
			828226800, 846370800, 859676400, 877820400, 891126000, 909270000, 922575600, 941324400,
			954025200, 972774000, 985474800, 1004223600, 1017529200, 1035673200, 1048978800,
			1067122800, 1080428400, 1099177200, 1111878000, 1130626800, 1143327600, 1162076400,
			1174777200, 1193526000, 1206831600, 1224975600, 1238281200, 1256425200, 1269730800,
			1288479600, 1301180400, 1414274400, 1459033200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 4, 1, 3, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 1, 3, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 11532, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+04", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const EUROPE_ATHENS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2344642492, -1686101632, -1182996000, -1178161200, -906861600, -904878000, -857257200,
			-844477200, -828237600, -812422800, -552362400, -541652400, 166485600, 186184800,
			198028800, 213753600, 228873600, 244080000, 260323200, 275446800, 291798000, 307407600,
			323388000, 338936400, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600,
			449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800,
			575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000,
			701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400,
			828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 5, 4, 5, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 5692, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5692, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 21 },
		],
		time_zone_designations: &["LMT", "AMT", "EEST", "EET", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_BRATISLAVA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3786829064, -2469401864, -1693706400, -1680483600, -1663455600, -1650150000,
			-1632006000, -1618700400, -938905200, -857257200, -844556400, -828226800, -812502000,
			-796777200, -781052400, -765327600, -746578800, -733359600, -728517600, -721260000,
			-716425200, -701910000, -684975600, -670460400, -654217200, -639010800, 291776400,
			307501200, 323830800, 338950800, 354675600, 370400400, 386125200, 401850000, 417574800,
			433299600, 449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000,
			559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200,
			686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600,
			811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3464, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3464, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: true, idx: 17 },
		],
		time_zone_designations: &["LMT", "PMT", "CEST", "CET", "GMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_BUCHAREST: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2469404664, -1213148664, -1187056800, -1175479200, -1159754400, -1144029600,
			-1127700000, -1111975200, -1096250400, -1080525600, -1064800800, -1049076000,
			-1033351200, -1017626400, -1001901600, -986176800, -970452000, -954727200, 296604000,
			307486800, 323816400, 338940000, 354672000, 370396800, 386121600, 401846400, 417571200,
			433296000, 449020800, 465350400, 481075200, 496800000, 512524800, 528249600, 543974400,
			559699200, 575424000, 591148800, 606873600, 622598400, 638323200, 654652800, 670370400,
			686095200, 701820000, 717544800, 733269600, 748994400, 764719200, 780440400, 796168800,
			811890000, 828223200, 846363600, 859683600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 6264, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 6264, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "BMT", "EEST", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_BUDAPEST: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2498260580, -1693706400, -1680483600, -1663455600, -1650150000, -1632006000,
			-1618700400, -1600470000, -1587250800, -1569711600, -1555196400, -906775200,
			-857257200, -844556400, -828226800, -812502000, -796777200, -778471200, -762656400,
			-749689200, -733276800, -717634800, -701910000, -686185200, -670460400, -654130800,
			-639010800, -492656400, -481168800, -461199600, -449708400, -428540400, -418258800,
			-397090800, -386809200, 323823600, 338943600, 354668400, 370393200, 386118000,
			401842800, 417567600, 433292400, 449024400, 465354000, 481078800, 496803600, 512528400,
			528253200, 543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800,
			654656400, 670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000,
			780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 4580, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_GIBRALTAR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2821649916, -1691964000, -1680472800, -1664143200, -1650146400, -1633903200,
			-1617487200, -1601848800, -1586037600, -1570399200, -1552168800, -1538344800,
			-1522533600, -1507500000, -1490565600, -1473631200, -1460930400, -1442786400,
			-1428876000, -1410732000, -1396216800, -1379282400, -1364767200, -1348437600,
			-1333317600, -1315778400, -1301263200, -1284328800, -1269813600, -1253484000,
			-1238364000, -1221429600, -1206914400, -1189980000, -1175464800, -1159135200,
			-1143410400, -1126476000, -1111960800, -1095631200, -1080511200, -1063576800,
			-1049061600, -1032127200, -1017612000, -1001282400, -986162400, -969228000, -950479200,
			-942012000, -904518000, -896050800, -875487600, -864601200, -844038000, -832546800,
			-812588400, -798073200, -781052400, -772066800, -764805600, -748476000, -733356000,
			-719445600, -717030000, -706748400, -699487200, -687996000, -668037600, -654732000,
			-636588000, -622072800, -605743200, -590623200, -574293600, -558568800, -542239200,
			-527119200, -512604000, -496274400, -481154400, -464220000, -449704800, -432165600,
			-417650400, -401320800, 386125200, 401850000, 417574800, 433299600, 449024400,
			465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800, 575427600,
			591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000, 701830800,
			717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 1, 2, 1, 2, 1, 3, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -1284, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 21 },
		],
		time_zone_designations: &["LMT", "BST", "GMT", "BDST", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_KALININGRAD: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2422056120, -1693706400, -1680483600, -1663455600, -1650150000, -1632006000,
			-1618700400, -938905200, -857257200, -844556400, -828226800, -812502000, -796777200,
			-781052400, -780372000, -778730400, -762663600, -749095200, 354920400, 370728000,
			386456400, 402264000, 417992400, 433800000, 449614800, 465346800, 481071600, 496796400,
			512521200, 528246000, 543970800, 559695600, 575420400, 591145200, 606870000, 622598400,
			638323200, 654652800, 670377600, 686102400, 701827200, 717552000, 733276800, 749001600,
			764726400, 780451200, 796176000, 811900800, 828230400, 846374400, 859680000, 877824000,
			891129600, 909273600, 922579200, 941328000, 954028800, 972777600, 985478400,
			1004227200, 1017532800, 1035676800, 1048982400, 1067126400, 1080432000, 1099180800,
			1111881600, 1130630400, 1143331200, 1162080000, 1174780800, 1193529600, 1206835200,
			1224979200, 1238284800, 1256428800, 1269734400, 1288483200, 1301184000, 1414278000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 4, 3, 4, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			6, 5, 6, 5, 6, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 7, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 4920, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 22 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 26 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 30 },
		],
		time_zone_designations: &["LMT", "CEST", "CET", "EEST", "EET", "MSD", "MSK", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: None,
	},
};
const EUROPE_KIROV: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1593820800, -1247540400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591141600, 606866400, 622594800, 638319600, 654649200, 670374000,
			701820000, 717548400, 733273200, 748998000, 764722800, 780447600, 796172400, 811897200,
			828226800, 846370800, 859676400, 877820400, 891126000, 909270000, 922575600, 941324400,
			954025200, 972774000, 985474800, 1004223600, 1017529200, 1035673200, 1048978800,
			1067122800, 1080428400, 1099177200, 1111878000, 1130626800, 1143327600, 1162076400,
			1174777200, 1193526000, 1206831600, 1224975600, 1238281200, 1256425200, 1269730800,
			1288479600, 1301180400, 1414274400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5, 4, 5, 3, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 11928, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+04", "MSD", "MSK", "MSK"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MSK", offset: -10800 },
		dst_info: None,
	},
};
const EUROPE_LUXEMBOURG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840141850, -2450995200, -1740355200, -1693702800, -1680483600, -1663455600,
			-1650150000, -1632006000, -1618700400, -1613826000, -1604278800, -1585530000,
			-1574038800, -1552266000, -1539997200, -1520557200, -1507510800, -1490576400,
			-1473642000, -1459126800, -1444006800, -1427677200, -1411952400, -1396227600,
			-1379293200, -1364778000, -1348448400, -1333328400, -1316394000, -1301263200,
			-1284328800, -1269813600, -1253484000, -1238364000, -1221429600, -1206914400,
			-1191189600, -1175464800, -1160344800, -1143410400, -1127685600, -1111960800,
			-1096840800, -1080511200, -1063576800, -1049061600, -1033336800, -1017612000,
			-1002492000, -986162400, -969228000, -950479200, -942012000, -934668000, -857257200,
			-844556400, -828226800, -812502000, -798073200, -781052400, -766623600, -745455600,
			-733273200, 228877200, 243997200, 260326800, 276051600, 291776400, 307501200,
			323830800, 338950800, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600,
			449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800,
			575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000,
			701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400,
			828234000,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 4, 3, 4, 3, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5,
			2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 4, 3, 4, 3, 4,
			3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 1050, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 1050, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 21 },
		],
		time_zone_designations: &["LMT", "BMT", "WET", "CET", "CEST", "WEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_MADRID: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2177452800, -1631926800, -1616889600, -1601168400, -1585353600, -1442451600,
			-1427673600, -1379293200, -1364774400, -1348448400, -1333324800, -1316390400,
			-1301270400, -1284339600, -1269820800, -1026954000, -1017619200, -1001898000,
			-999482400, -986090400, -954115200, -940208400, -873079200, -862621200, -842839200,
			-828320400, -811389600, -796870800, -779940000, -765421200, -748490400, -733971600,
			-652327200, -639018000, 135122400, 150246000, 166572000, 181695600, 196812000,
			212540400, 228866400, 243990000, 260326800, 276051600, 291776400, 307501200, 323830800,
			338950800, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600, 449024400,
			465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800, 575427600,
			591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000, 701830800,
			717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 2, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -884, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 23 },
		],
		time_zone_designations: &["LMT", "WEST", "WET", "WEMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_MALTA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2403478684, -1690765200, -1680487200, -1664758800, -1648951200, -1635123600,
			-1616896800, -1604278800, -1585533600, -1571014800, -1555293600, -932432400,
			-857257200, -844556400, -828226800, -812588400, -798073200, -781052400, -766717200,
			-750898800, -733359600, -719456400, -701917200, -689209200, -670460400, -114051600,
			-103168800, -81997200, -71715600, -50547600, -40266000, -18493200, -8211600, 12956400,
			23238000, 43801200, 54687600, 75855600, 86742000, 102380400, 118105200, 135730800,
			148518000, 167187600, 180489600, 198637200, 211939200, 230086800, 243388800, 261536400,
			274838400, 292986000, 306288000, 323312400, 338342400, 354675600, 370400400, 386125200,
			401850000, 417574800, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400,
			528253200, 543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800,
			654656400, 670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000,
			780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3484, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_MARIEHAMN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2890258789, -1535938789, -875671200, -859773600, 354672000, 370396800, 386121600,
			401846400, 417574800, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400,
			528253200, 543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800,
			654656400, 670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000,
			780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 5989, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5989, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "HMT", "EEST", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_MINSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840147416, -1441158600, -1247536800, -899780400, -857257200, -844556400, -828226800,
			-812502000, -804650400, 354920400, 370728000, 386456400, 402264000, 417992400,
			433800000, 449614800, 465346800, 481071600, 496796400, 512521200, 528246000, 543970800,
			559695600, 575420400, 591145200, 606870000, 622594800, 670374000, 686102400, 701827200,
			717552000, 733276800, 749001600, 764726400, 780451200, 796176000, 811900800, 828230400,
			846374400, 859680000, 877824000, 891129600, 909273600, 922579200, 941328000, 954028800,
			972777600, 985478400, 1004227200, 1017532800, 1035676800, 1048982400, 1067126400,
			1080432000, 1099180800, 1111881600, 1130630400, 1143331200, 1162080000, 1174780800,
			1193529600, 1206835200, 1224979200, 1238284800, 1256428800, 1269734400, 1288483200,
			1301184000,
		],
		transition_types: &[
			1, 2, 3, 5, 4, 5, 4, 5, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 7, 2,
			7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7,
			2, 7, 2, 7, 2, 7, 2, 7, 2, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 6616, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 6600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 25 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 34 },
		],
		time_zone_designations: &["LMT", "MMT", "EET", "MSK", "CET", "CEST", "MSD", "EEST", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const EUROPE_MONACO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2486592561, -1855958961, -1689814800, -1680397200, -1665363600, -1648342800,
			-1635123600, -1616893200, -1604278800, -1585443600, -1574038800, -1552266000,
			-1539997200, -1520557200, -1507510800, -1490576400, -1470618000, -1459126800,
			-1444006800, -1427677200, -1411952400, -1396227600, -1379293200, -1364778000,
			-1348448400, -1333328400, -1316394000, -1301274000, -1284339600, -1269824400,
			-1253494800, -1238374800, -1221440400, -1206925200, -1191200400, -1175475600,
			-1160355600, -1143421200, -1127696400, -1111971600, -1096851600, -1080522000,
			-1063587600, -1049072400, -1033347600, -1017622800, -1002502800, -986173200,
			-969238800, -950490000, -942012000, -932436000, -857257200, -844556400, -828226800,
			-812502000, -800071200, -796266000, -781052400, -766623600, 196819200, 212540400,
			228877200, 243997200, 260326800, 276051600, 291776400, 307501200, 323830800, 338950800,
			354675600, 370400400, 386125200, 401850000, 417574800, 433299600, 449024400, 465354000,
			481078800, 496803600, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400,
			606877200, 622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600,
			733280400, 749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 5, 4, 5, 4, 5, 6, 2,
			6, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 561, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 561, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 21 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 26 },
		],
		time_zone_designations: &["LMT", "PMT", "WEST", "WET", "CET", "CEST", "WEMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_NICOSIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1518920008, 166572000, 182293200, 200959200, 213829200, 228866400, 243982800,
			260316000, 276123600, 291765600, 307486800, 323820000, 338936400, 354664800, 370386000,
			386114400, 401835600, 417564000, 433285200, 449013600, 465339600, 481068000, 496789200,
			512517600, 528238800, 543967200, 559688400, 575416800, 591138000, 606866400, 622587600,
			638316000, 654642000, 670370400, 686091600, 701820000, 717541200, 733269600, 748990800,
			764719200, 780440400, 796168800, 811890000, 828223200, 843944400, 859672800, 875394000,
			891122400, 909277200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8008, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "EEST", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_RIGA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840146594, -1632008194, -1618702594, -1601681794, -1597275394, -1377308194,
			-928029600, -899521200, -857257200, -844556400, -828226800, -812502000, -796777200,
			-795834000, 354920400, 370728000, 386456400, 402264000, 417992400, 433800000,
			449614800, 465346800, 481071600, 496796400, 512521200, 528246000, 543970800, 559695600,
			575420400, 591145200, 606870000, 622598400, 638323200, 654652800, 670377600, 686102400,
			701827200, 717552000, 733276800, 749001600, 764726400, 780451200, 796176000, 811900800,
			828230400, 843955200, 859683600, 877827600, 891133200, 909277200, 922582800, 941331600,
			985482000,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 3, 4, 6, 5, 6, 5, 6, 5, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7,
			4, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8, 3, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 5794, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5794, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 9394, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 24 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 33 },
		],
		time_zone_designations: &["LMT", "RMT", "LST", "EET", "MSK", "CET", "CEST", "MSD", "EEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_SAMARA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1593820800, -1247540400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591141600, 606866400, 622594800, 638319600, 654649200, 670374000,
			686102400, 687916800, 701820000, 717544800, 733269600, 748994400, 764719200, 780444000,
			796168800, 811893600, 828223200, 846367200, 859672800, 877816800, 891122400, 909266400,
			922572000, 941320800, 954021600, 972770400, 985471200, 1004220000, 1017525600,
			1035669600, 1048975200, 1067119200, 1080424800, 1099173600, 1111874400, 1130623200,
			1143324000, 1162072800, 1174773600, 1193522400, 1206828000, 1224972000, 1238277600,
			1256421600, 1269727200, 1288479600, 1301180400,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 1, 4, 1, 5, 1, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 4, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 12020, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
		],
		time_zone_designations: &["LMT", "+03", "+04", "+05", "+04", "+03"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const EUROPE_SARATOV: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1593820800, -1247540400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591145200, 606870000, 622594800, 638319600, 654649200, 670374000,
			701820000, 717548400, 733273200, 748998000, 764722800, 780447600, 796172400, 811897200,
			828226800, 846370800, 859676400, 877820400, 891126000, 909270000, 922575600, 941324400,
			954025200, 972774000, 985474800, 1004223600, 1017529200, 1035673200, 1048978800,
			1067122800, 1080428400, 1099177200, 1111878000, 1130626800, 1143327600, 1162076400,
			1174777200, 1193526000, 1206831600, 1224975600, 1238281200, 1256425200, 1269730800,
			1288479600, 1301180400, 1414274400, 1480806000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 4, 1, 4, 1, 3, 4, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 1, 3, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 11058, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+04", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const EUROPE_SIMFEROPOL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840148984, -1441160160, -1247536800, -888894000, -857257200, -844556400, -828226800,
			-812502000, -811648800, 354920400, 370728000, 386456400, 402264000, 417992400,
			433800000, 449614800, 465346800, 481071600, 496796400, 512521200, 528246000, 543970800,
			559695600, 575420400, 591145200, 606870000, 622594800, 646786800, 701827200, 717552000,
			733276800, 749001600, 764726400, 767739600, 780447600, 796172400, 811897200, 828219600,
			846374400, 859683600, 877827600, 891133200, 909277200, 922582800, 941331600, 954032400,
			972781200, 985482000, 1004230800, 1017536400, 1035680400, 1048986000, 1067130000,
			1080435600, 1099184400, 1111885200, 1130634000, 1143334800, 1162083600, 1174784400,
			1193533200, 1206838800, 1224982800, 1238288400, 1256432400, 1269738000, 1288486800,
			1301187600, 1319936400, 1332637200, 1351386000, 1364691600, 1382835600, 1396137600,
			1414274400,
		],
		transition_types: &[
			1, 2, 3, 5, 4, 5, 4, 5, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 2, 7,
			2, 7, 2, 7, 6, 3, 6, 3, 6, 3, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7,
			2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 7, 2, 8, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8184, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 8160, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 25 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["LMT", "SMT", "EET", "MSK", "CET", "CEST", "MSD", "EEST", "MSK"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MSK", offset: -10800 },
		dst_info: None,
	},
};
const EUROPE_SOFIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840146396, -2369527016, -857257200, -844556400, -828226800, -812502000, -796777200,
			-781048800, 291762000, 307576800, 323816400, 339026400, 355266000, 370393200,
			386715600, 401846400, 417571200, 433296000, 449020800, 465350400, 481075200, 496800000,
			512524800, 528249600, 543974400, 559699200, 575424000, 591148800, 606873600, 622598400,
			638323200, 654652800, 670370400, 686091600, 701820000, 717541200, 733269600, 748990800,
			764719200, 780440400, 796168800, 811890000, 828223200, 846363600, 859683600,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 4, 3, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5,
			2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 5596, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7016, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 21 },
		],
		time_zone_designations: &["LMT", "IMT", "EET", "CET", "CEST", "EEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_TALLINN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840146740, -1638322740, -1632006000, -1618700400, -1593824400, -1535938740,
			-927943200, -892954800, -857257200, -844556400, -828226800, -812502000, -797652000,
			354920400, 370728000, 386456400, 402264000, 417992400, 433800000, 449614800, 465346800,
			481071600, 496796400, 512521200, 528246000, 543970800, 559695600, 575420400, 591145200,
			606870000, 622598400, 638323200, 654652800, 670377600, 686102400, 701827200, 717552000,
			733276800, 749001600, 764726400, 780451200, 796176000, 811900800, 828230400, 846374400,
			859680000, 877824000, 891129600, 909277200, 922582800, 941331600, 1017536400,
		],
		transition_types: &[
			1, 3, 2, 3, 1, 4, 5, 2, 3, 2, 3, 2, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
			7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7, 4, 7,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 5940, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5940, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 21 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 25 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 29 },
		],
		time_zone_designations: &["LMT", "TMT", "CEST", "CET", "EET", "MSK", "MSD", "EEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_TIRANE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1767230360, -932346000, -857257200, -844556400, -843519600, 136854000, 149896800,
			168130800, 181432800, 199839600, 213141600, 231894000, 244591200, 263257200, 276040800,
			294706800, 307490400, 326156400, 339458400, 357087600, 370389600, 389142000, 402444000,
			419468400, 433807200, 449622000, 465354000, 481078800, 496803600, 512528400, 528253200,
			543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400,
			670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800,
			796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 4760, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_TIRASPOL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840147720, -1637114100, -1213148664, -1187056800, -1175479200, -1159754400,
			-1144029600, -1127700000, -1111975200, -1096250400, -1080525600, -1064800800,
			-1049076000, -1033351200, -1017626400, -1001901600, -986176800, -970452000, -954727200,
			-927165600, -898138800, -857257200, -844556400, -828226800, -812502000, -800157600,
			354920400, 370728000, 386456400, 402264000, 417992400, 433800000, 449614800, 465346800,
			481071600, 496796400, 512521200, 528246000, 543970800, 559695600, 575420400, 591145200,
			606870000, 622594800, 638319600, 641944800, 654652800, 670377600, 686102400, 701820000,
			717541200, 733269600, 748990800, 764719200, 780440400, 796168800, 811890000, 828223200,
			846363600, 859680000,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 6, 5, 6, 5, 6, 8, 7, 8, 7,
			8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
			4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 6920, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 6900, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 6264, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 21 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 25 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 30 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 34 },
		],
		time_zone_designations: &["LMT", "CMT", "BMT", "EEST", "EET", "CET", "CEST", "MSD", "MSK"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_ULYANOVSK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1593820800, -1247540400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591141600, 606866400, 622594800, 638319600, 654649200, 670374000,
			686102400, 695779200, 701823600, 717548400, 733273200, 748998000, 764722800, 780447600,
			796172400, 811897200, 828226800, 846370800, 859676400, 877820400, 891126000, 909270000,
			922575600, 941324400, 954025200, 972774000, 985474800, 1004223600, 1017529200,
			1035673200, 1048978800, 1067122800, 1080428400, 1099177200, 1111878000, 1130626800,
			1143327600, 1162076400, 1174777200, 1193526000, 1206831600, 1224975600, 1238281200,
			1256425200, 1269730800, 1288479600, 1301180400, 1414274400, 1459033200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 1, 4, 1, 5, 6, 1, 4, 1, 4, 1,
			4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4,
			1, 4, 1, 4, 1, 3, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 11616, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+03", "+05", "+04", "+04", "+03", "+02"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const EUROPE_VADUZ: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3675198848, -2385246586, -904435200, -891129600, -872985600, -859680000, 354675600,
			370400400, 386125200, 401850000, 417574800, 433299600, 449024400, 465354000, 481078800,
			496803600, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400, 606877200,
			622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600, 733280400,
			749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 2048, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 1786, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "BMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_VATICAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3252098996, -2403565200, -1690765200, -1680487200, -1664758800, -1648951200,
			-1635123600, -1616896800, -1604278800, -1585533600, -1571014800, -1555293600,
			-932432400, -857257200, -844556400, -828226800, -812502000, -798073200, -781052400,
			-766717200, -750898800, -733359600, -719456400, -701917200, -689209200, -670460400,
			-114051600, -103168800, -81997200, -71715600, -50547600, -40266000, -18493200,
			-8211600, 12956400, 23238000, 43801200, 54687600, 75855600, 86742000, 107910000,
			118191600, 138754800, 149641200, 170809200, 181090800, 202258800, 212540400, 233103600,
			243990000, 265158000, 276044400, 296607600, 307494000, 323830800, 338950800, 354675600,
			370400400, 386125200, 401850000, 417574800, 433299600, 449024400, 465354000, 481078800,
			496803600, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400, 606877200,
			622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600, 733280400,
			749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 2996, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 2996, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "RMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_VIENNA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2422055121, -1693706400, -1680483600, -1663455600, -1650150000, -1632006000,
			-1618700400, -1569711600, -1555801200, -938905200, -857257200, -844556400, -828226800,
			-812502000, -796777200, -781052400, -780188400, -748479600, -733273200, -717634800,
			-701910000, -684975600, -670460400, 323823600, 338940000, 354675600, 370400400,
			386125200, 401850000, 417574800, 433299600, 449024400, 465354000, 481078800, 496803600,
			512528400, 528253200, 543978000, 559702800, 575427600, 591152400, 606877200, 622602000,
			638326800, 654656400, 670381200, 686106000, 701830800, 717555600, 733280400, 749005200,
			764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3921, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "CEST", "CET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_VILNIUS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840146876, -1672536240, -1585100136, -1561251600, -1553565600, -928198800,
			-900126000, -857257200, -844556400, -828226800, -812502000, -802144800, 354920400,
			370728000, 386456400, 402264000, 417992400, 433800000, 449614800, 465346800, 481071600,
			496796400, 512521200, 528246000, 543970800, 559695600, 575420400, 591145200, 606870000,
			622598400, 638323200, 654652800, 670377600, 686102400, 701827200, 717552000, 733276800,
			749001600, 764726400, 780451200, 796176000, 811900800, 828230400, 846374400, 859680000,
			877824000, 891133200, 909277200, 922582800, 941331600, 1048986000,
		],
		transition_types: &[
			1, 2, 3, 4, 3, 5, 6, 3, 6, 3, 6, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 7, 5, 8,
			4, 8, 4, 8, 4, 8, 4, 8, 4, 8, 4, 8, 4, 8, 4, 8, 4, 6, 3, 6, 4, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 6076, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5040, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 5736, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 24 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 33 },
		],
		time_zone_designations: &["LMT", "WMT", "KMT", "CET", "EET", "MSK", "CEST", "MSD", "EEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const EUROPE_VOLGOGRAD: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1577761060, -1247540400, 354916800, 370724400, 386452800, 402260400, 417988800,
			433796400, 449611200, 465343200, 481068000, 496792800, 512517600, 528242400, 543967200,
			559692000, 575416800, 591145200, 606870000, 622594800, 638319600, 654649200, 670374000,
			701820000, 717548400, 733273200, 748998000, 764722800, 780447600, 796172400, 811897200,
			828226800, 846370800, 859676400, 877820400, 891126000, 909270000, 922575600, 941324400,
			954025200, 972774000, 985474800, 1004223600, 1017529200, 1035673200, 1048978800,
			1067122800, 1080428400, 1099177200, 1111878000, 1130626800, 1143327600, 1162076400,
			1174777200, 1193526000, 1206831600, 1224975600, 1238281200, 1256425200, 1269730800,
			1288479600, 1301180400, 1414274400, 1540681200, 1609020000,
		],
		transition_types: &[
			1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 5, 4, 5, 4, 5, 2, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 6, 5, 2, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 10660, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "+03", "+04", "+05", "MSD", "MSK", "MSK"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MSK", offset: -10800 },
		dst_info: None,
	},
};
const EUROPE_ZAGREB: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713915320, -905824800, -857257200, -844556400, -828226800, -812502000, -796777200,
			-777942000, -766623600, 417574800, 433299600, 449024400, 465354000, 481078800,
			496803600, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400, 606877200,
			622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600, 733280400,
			749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 4920, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
		],
		time_zone_designations: &["LMT", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const EUROPE_ZAPOROZHYE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840148124, -1441159324, -1247536800, -892522800, -857257200, -844556400, -828226800,
			-825382800, 354920400, 370728000, 386456400, 402264000, 417992400, 433800000,
			449614800, 465346800, 481071600, 496796400, 512521200, 528246000, 543970800, 559695600,
			575420400, 591145200, 606870000, 622594800, 638319600, 646783200, 686102400, 701827200,
			717552000, 733276800, 749001600, 764726400, 780451200, 796176000, 811900800, 828230400,
			846378000,
		],
		transition_types: &[
			1, 2, 3, 5, 4, 5, 4, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 7, 2,
			7, 2, 7, 2, 7, 2, 7, 2, 7, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 7324, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7324, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 25 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 29 },
		],
		time_zone_designations: &["LMT", "KMT", "EET", "MSK", "CET", "CEST", "MSD", "EEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EEST", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 10800 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 14400 },
		}),
	},
};
const FACTORY: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 }],
		time_zone_designations: &["-00"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "-00", offset: 0 }, dst_info: None },
};
const GB_EIRE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3852662325, -1691964000, -1680472800, -1664143200, -1650146400, -1633903200,
			-1617487200, -1601848800, -1586037600, -1570399200, -1552168800, -1538344800,
			-1522533600, -1507500000, -1490565600, -1473631200, -1460930400, -1442786400,
			-1428876000, -1410732000, -1396216800, -1379282400, -1364767200, -1348437600,
			-1333317600, -1315778400, -1301263200, -1284328800, -1269813600, -1253484000,
			-1238364000, -1221429600, -1206914400, -1189980000, -1175464800, -1159135200,
			-1143410400, -1126476000, -1111960800, -1095631200, -1080511200, -1063576800,
			-1049061600, -1032127200, -1017612000, -1001282400, -986162400, -969228000, -950479200,
			-942012000, -904518000, -896050800, -875487600, -864601200, -844038000, -832546800,
			-812588400, -798073200, -781052400, -772066800, -764805600, -748476000, -733356000,
			-719445600, -717030000, -706748400, -699487200, -687996000, -668037600, -654732000,
			-636588000, -622072800, -605743200, -590623200, -574293600, -558568800, -542239200,
			-527119200, -512604000, -496274400, -481154400, -464220000, -449704800, -432165600,
			-417650400, -401320800, -386200800, -369266400, -354751200, -337816800, -323301600,
			-306972000, -291852000, -276732000, -257983200, -245282400, -226533600, -213228000,
			-195084000, -182383200, -163634400, -150933600, -132184800, -119484000, -100735200,
			-88034400, -68680800, -59004000, -37242000, 57722400, 69818400, 89172000, 101268000,
			120621600, 132717600, 152071200, 164167200, 183520800, 196221600, 214970400, 227671200,
			246420000, 259120800, 278474400, 290570400, 309924000, 322020000, 341373600, 354675600,
			372819600, 386125200, 404269200, 417574800, 435718800, 449024400, 467773200, 481078800,
			499222800, 512528400, 530672400, 543978000, 562122000, 575427600, 593571600, 606877200,
			625626000, 638326800, 657075600, 670381200, 688525200, 701830800, 719974800, 733280400,
			751424400, 764730000, 782874000, 796179600, 814323600, 828234000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 1, 2, 1, 2, 1, 3, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 4, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -75, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "BST", "GMT", "BDST", "BST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "GMT", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "BST", offset: -3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const GREENWICH: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 }],
		time_zone_designations: &["GMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "GMT", offset: 0 }, dst_info: None },
};
const HST: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -36000,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["HST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "HST", offset: 36000 },
		dst_info: None,
	},
};
const HONGKONG: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2056690800, -900910800, -891579600, -884248200, -761209200, -747907200, -728541000,
			-717049800, -697091400, -683785800, -668061000, -654755400, -636611400, -623305800,
			-605161800, -591856200, -573712200, -559801800, -541657800, -528352200, -510211800,
			-498112200, -478762200, -466662600, -446707800, -435213000, -415258200, -403158600,
			-383808600, -371709000, -352359000, -340259400, -320909400, -308809800, -288855000,
			-277360200, -257405400, -245910600, -225955800, -213856200, -194506200, -182406600,
			-163056600, -148537800, -132816600, -117088200, -101367000, -85638600, -69312600,
			-53584200, -37863000, -22134600, -6413400, 9315000, 25036200, 40764600, 56485800,
			72214200, 88540200, 104268600, 119989800, 126041400, 151439400, 167167800, 182889000,
			198617400, 214338600, 295385400, 309292200,
		],
		transition_types: &[
			1, 2, 3, 4, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 27402, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 30600, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 18 },
		],
		time_zone_designations: &["LMT", "HKT", "HKST", "HKWT", "JST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "HKT", offset: -28800 },
		dst_info: None,
	},
};
const ICELAND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1830383032],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -968, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "GMT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "GMT", offset: 0 }, dst_info: None },
};
const INDIAN_CHAGOS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1988167780, 820436400],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 17380, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 21600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+05", "+06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+06", offset: -21600 },
		dst_info: None,
	},
};
const INDIAN_KERGUELEN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2840158440, -315636840],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 17640, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 17640, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "MMT", "+05"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+05", offset: -18000 },
		dst_info: None,
	},
};
const INDIAN_MAURITIUS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1988164200, 403041600, 417034800, 1224972000, 1238274000],
		transition_types: &[2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 13800, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+05", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+04", offset: -14400 },
		dst_info: None,
	},
};
const INDIAN_MAYOTTE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1946168836, -1309746600, -1261969200, -1041388200, -865305900],
		transition_types: &[1, 2, 1, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8836, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 9000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 9900, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "+0230", "EAT", "+0245"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EAT", offset: -10800 },
		dst_info: None,
	},
};
const IRAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1704165944, -1090466744, 227820600, 246223800, 259617600, 271108800, 283982400,
			296598600, 306531000, 322432200, 338499000, 673216200, 685481400, 701209800, 717103800,
			732745800, 748639800, 764281800, 780175800, 795817800, 811711800, 827353800, 843247800,
			858976200, 874870200, 890512200, 906406200, 922048200, 937942200, 953584200, 969478200,
			985206600, 1001100600, 1016742600, 1032636600, 1048278600, 1064172600, 1079814600,
			1095708600, 1111437000, 1127331000, 1206045000, 1221939000, 1237667400, 1253561400,
			1269203400, 1285097400, 1300739400, 1316633400, 1332275400, 1348169400, 1363897800,
			1379791800, 1395433800, 1411327800, 1426969800, 1442863800, 1458505800, 1474399800,
			1490128200, 1506022200, 1521664200, 1537558200, 1553200200, 1569094200, 1584736200,
			1600630200, 1616358600, 1632252600, 1647894600, 1663788600,
		],
		transition_types: &[
			1, 3, 2, 5, 4, 5, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 12344, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 12344, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 16200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 12600, is_dst: false, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 24 },
		],
		time_zone_designations: &["LMT", "TMT", "+0430", "+0330", "+05", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+0330", offset: -12600 },
		dst_info: None,
	},
};
const ISRAEL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840149254, -1641003640, -933638400, -923097600, -919036800, -857347200, -844300800,
			-825811200, -812678400, -794188800, -779846400, -762652800, -748310400, -731116800,
			-681955200, -673228800, -667958400, -652320000, -636422400, -622080000, -608947200,
			-591840000, -572486400, -558576000, -542851200, -527731200, -514425600, -490838400,
			-482976000, -459388800, -451526400, -428544000, -418262400, -400118400, -387417600,
			142380000, 150843600, 167176800, 178664400, 334101600, 337730400, 452642400, 462319200,
			482277600, 494370000, 516751200, 526424400, 545436000, 558478800, 576626400, 589323600,
			609890400, 620773200, 638316000, 651618000, 669765600, 683672400, 701820000, 715726800,
			733701600, 747176400, 765151200, 778021200, 796600800, 810075600, 826840800, 842821200,
			858895200, 874184400, 890344800, 905029200, 923011200, 936313200, 955670400, 970783200,
			986770800, 1001282400, 1017356400, 1033941600, 1048806000, 1065132000, 1081292400,
			1095804000, 1112313600, 1128812400, 1143763200, 1159657200, 1175212800, 1189897200,
			1206662400, 1223161200, 1238112000, 1254006000, 1269561600, 1284246000, 1301616000,
			1317510000, 1333065600, 1348354800, 1364515200,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 4, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 8454, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 8440, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "JMT", "IDT", "IST", "IDDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "IST", offset: -7200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "IDT", offset: -10800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 4, 4), time: 93600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const JAMAICA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2524503170, -1827687170, 126687600, 152085600, 162370800, 183535200, 199263600,
			215589600, 230713200, 247039200, 262767600, 278488800, 294217200, 309938400, 325666800,
			341388000, 357116400, 372837600, 388566000, 404892000, 420015600, 436341600,
		],
		transition_types: &[1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -18430, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18430, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "KMT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: None,
	},
};
const JAPAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2587712400, -683802000, -672310800, -654771600, -640861200, -620298000, -609411600,
			-588848400, -577962000,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 33539, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "JDT", "JST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "JST", offset: -32400 },
		dst_info: None,
	},
};
const KWAJALEIN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2177492960, -1041418800, -907408800, -817462800, -7988400, 745934400],
		transition_types: &[1, 2, 3, 1, 4, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 40160, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -43200, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "+11", "+10", "+09", "-12", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const LIBYA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1577926364, -574902000, -568087200, -512175600, -504928800, -449888400, -441856800,
			-347158800, 378684000, 386463600, 402271200, 417999600, 433807200, 449622000,
			465429600, 481590000, 496965600, 512953200, 528674400, 544230000, 560037600, 575852400,
			591660000, 607388400, 623196000, 641775600, 844034400, 860108400, 875916000,
			1352505600, 1364515200, 1382659200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 3, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 1, 3,
			2, 1, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3164, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 13 },
		],
		time_zone_designations: &["LMT", "CEST", "CET", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EET", offset: -7200 },
		dst_info: None,
	},
};
const MET: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1693706400, -1680483600, -1663455600, -1650150000, -1632006000, -1618700400,
			-938905200, -857257200, -844556400, -828226800, -812502000, -796777200, -781052400,
			-766623600, 228877200, 243997200, 260326800, 276051600, 291776400, 307501200,
			323830800, 338950800, 354675600, 370400400, 386125200, 401850000, 417574800, 433299600,
			449024400, 465354000, 481078800, 496803600, 512528400, 528253200, 543978000, 559702800,
			575427600, 591152400, 606877200, 622602000, 638326800, 654656400, 670381200, 686106000,
			701830800, 717555600, 733280400, 749005200, 764730000, 780454800, 796179600, 811904400,
			828234000,
		],
		transition_types: &[
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 5 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 0 },
		],
		time_zone_designations: &["MET", "MEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const MST: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord {
			utc_offset: -25200,
			is_dst: false,
			idx: 0,
		}],
		time_zone_designations: &["MST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const MST_7_MDT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1633273200, -1615132800, -1601823600, -1583683200, -880210800, -769395600, -765388800,
			-84380400, -68659200, -52930800, -37209600, -21481200, -5760000, 9968400, 25689600,
			41418000, 57744000, 73472400, 89193600, 104922000, 120643200, 126694800, 152092800,
			162378000, 183542400, 199270800, 215596800, 230720400, 247046400, 262774800, 278496000,
			294224400, 309945600, 325674000, 341395200, 357123600, 372844800, 388573200, 404899200,
			420022800, 436348800, 452077200, 467798400, 483526800, 499248000, 514976400, 530697600,
			544611600, 562147200, 576061200, 594201600, 607510800, 625651200, 638960400, 657100800,
			671014800, 688550400, 702464400, 720000000, 733914000, 752054400, 765363600, 783504000,
			796813200, 814953600, 828867600, 846403200, 860317200, 877852800, 891766800, 909302400,
			923216400, 941356800, 954666000, 972806400, 986115600, 1004256000, 1018170000,
			1035705600, 1049619600, 1067155200, 1081069200, 1099209600, 1112518800, 1130659200,
			1143968400, 1162108800, 1173603600,
		],
		transition_types: &[
			1, 0, 1, 0, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["MST", "MDT", "MWT", "MPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const MEXICO_BAJA_NORTE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1451667600, -1343062800, -1234803600, -1222963200, -1207242000,
			-873820800, -769395600, -761677200, -686073600, -661539600, -495039600, -481734000,
			-463590000, -450284400, -431535600, -418230000, -400086000, -386780400, -368636400,
			-355330800, -337186800, -323881200, -305737200, -292431600, 199274400, 215600400,
			230724000, 247050000, 262778400, 278499600, 294228000, 309949200, 325677600, 341398800,
			357127200, 372848400, 388576800, 404902800, 420026400, 436352400, 452080800, 467802000,
			483530400, 499251600, 514980000, 530701200, 544615200, 562150800, 576064800, 594205200,
			607514400, 625654800, 638964000, 657104400, 671018400, 688554000, 702468000, 720003600,
			733917600, 752058000, 765367200, 783507600, 796816800, 814957200, 828871200, 846406800,
			860320800, 877856400, 891770400, 909306000, 923220000, 941360400, 954669600, 972810000,
			986119200, 1004259600, 1018173600, 1035709200, 1049623200, 1067158800, 1081072800,
			1099213200, 1112522400, 1130662800, 1143972000, 1162112400, 1175421600, 1193562000,
			1207476000, 1225011600, 1238925600, 1256461200, 1268560800,
		],
		transition_types: &[
			1, 2, 1, 2, 3, 2, 4, 5, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -28084, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "MST", "PST", "PDT", "PWT", "PPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "PST", offset: 28800 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "PDT", offset: 25200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const MEXICO_BAJA_SUR: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			-873828000, -661539600, 28800, 828867600, 846403200, 860317200, 877852800, 891766800,
			909302400, 923216400, 941356800, 954666000, 972806400, 989139600, 1001836800,
			1018170000, 1035705600, 1049619600, 1067155200, 1081069200, 1099209600, 1112518800,
			1130659200, 1143968400, 1162108800, 1175418000, 1193558400, 1207472400, 1225008000,
			1238922000, 1256457600, 1270371600, 1288512000, 1301821200, 1319961600, 1333270800,
			1351411200, 1365325200, 1382860800, 1396774800, 1414310400, 1428224400, 1445760000,
			1459674000, 1477814400, 1491123600, 1509264000, 1522573200, 1540713600, 1554627600,
			1572163200, 1586077200, 1603612800, 1617526800, 1635667200, 1648976400, 1667116800,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 2, 1, 4, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3,
			1, 3, 1, 3, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25540, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "PST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const MEXICO_GENERAL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1514739600, -1343066400, -1234807200, -1220292000, -1207159200, -1191344400,
			-975261600, -963169200, -917114400, -907354800, -821901600, -810068400, -627501600,
			-612990000, 828864000, 846399600, 860313600, 877849200, 891763200, 909298800,
			923212800, 941353200, 954662400, 972802800, 989136000, 1001833200, 1018166400,
			1035702000, 1049616000, 1067151600, 1081065600, 1099206000, 1112515200, 1130655600,
			1143964800, 1162105200, 1175414400, 1193554800, 1207468800, 1225004400, 1238918400,
			1256454000, 1270368000, 1288508400, 1301817600, 1319958000, 1333267200, 1351407600,
			1365321600, 1382857200, 1396771200, 1414306800, 1428220800, 1445756400, 1459670400,
			1477810800, 1491120000, 1509260400, 1522569600, 1540710000, 1554624000, 1572159600,
			1586073600, 1603609200, 1617523200, 1635663600, 1648972800, 1667113200,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 2, 4, 2, 4, 2, 5, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4,
			2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2,
			4, 2, 4, 2, 4, 2, 4, 2, 4, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -23796, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "MST", "CST", "MDT", "CDT", "CWT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: None,
	},
};
const NZ: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3192435544, -1330335000, -1320057000, -1300699800, -1287396000, -1269250200,
			-1255946400, -1237800600, -1224496800, -1206351000, -1192442400, -1174901400,
			-1160992800, -1143451800, -1125914400, -1112607000, -1094464800, -1081157400,
			-1063015200, -1049707800, -1031565600, -1018258200, -1000116000, -986808600,
			-968061600, -955359000, -936612000, -923304600, -757425600, 152632800, 162309600,
			183477600, 194968800, 215532000, 226418400, 246981600, 257868000, 278431200, 289317600,
			309880800, 320767200, 341330400, 352216800, 372780000, 384271200, 404834400, 415720800,
			436284000, 447170400, 467733600, 478620000, 499183200, 510069600, 530632800, 541519200,
			562082400, 573573600, 594136800, 605023200, 623772000, 637682400, 655221600, 669132000,
			686671200, 700581600, 718120800, 732636000, 749570400, 764085600, 781020000, 795535200,
			812469600, 826984800, 844524000, 858434400, 875973600, 889884000, 907423200, 921938400,
			938872800, 953388000, 970322400, 984837600, 1002376800, 1016287200, 1033826400,
			1047736800, 1065276000, 1079791200, 1096725600, 1111240800, 1128175200, 1142690400,
			1159624800, 1174140000, 1191074400,
		],
		transition_types: &[
			2, 1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4,
			5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,
			4, 5, 4, 5, 4, 5, 4, 5, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 41944, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 45000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 41400, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: true, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "NZST", "NZMT", "NZST", "NZDT", "NZST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "NZST", offset: -43200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "NZDT", offset: -46800 },
			start_date: TransitionDate { day: MonthWeekDay(9, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const NZ_CHAT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3192437628, -757426500, 152632800, 162309600, 183477600, 194968800, 215532000,
			226418400, 246981600, 257868000, 278431200, 289317600, 309880800, 320767200, 341330400,
			352216800, 372780000, 384271200, 404834400, 415720800, 436284000, 447170400, 467733600,
			478620000, 499183200, 510069600, 530632800, 541519200, 562082400, 573573600, 594136800,
			605023200, 623772000, 637682400, 655221600, 669132000, 686671200, 700581600, 718120800,
			732636000, 749570400, 764085600, 781020000, 795535200, 812469600, 826984800, 844524000,
			858434400, 875973600, 889884000, 907423200, 921938400, 938872800, 953388000, 970322400,
			984837600, 1002376800, 1016287200, 1033826400, 1047736800, 1065276000, 1079791200,
			1096725600, 1111240800, 1128175200, 1142690400, 1159624800, 1174140000, 1191074400,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 44028, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 44100, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 49500, is_dst: true, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 45900, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "+1215", "+1345", "+1245"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+1245", offset: -45900 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "+1345", offset: -49500 },
			start_date: TransitionDate { day: MonthWeekDay(9, 5, 0), time: 9900 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 13500 },
		}),
	},
};
const PRC: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2177481943, -1600675200, -1585904400, -933667200, -922093200, -908870400, -888829200,
			-881049600, -767869200, -745833600, -733827600, -716889600, -699613200, -683884800,
			-670669200, -652348800, -650019600, 515527200, 527014800, 545162400, 558464400,
			577216800, 589914000, 608666400, 621968400, 640116000, 653418000, 671565600, 684867600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 29143, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "CDT", "CST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: -28800 },
		dst_info: None,
	},
};
const PST_8_PDT: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1633269600, -1615129200, -1601820000, -1583679600, -880207200, -769395600, -765385200,
			-84376800, -68655600, -52927200, -37206000, -21477600, -5756400, 9972000, 25693200,
			41421600, 57747600, 73476000, 89197200, 104925600, 120646800, 126698400, 152096400,
			162381600, 183546000, 199274400, 215600400, 230724000, 247050000, 262778400, 278499600,
			294228000, 309949200, 325677600, 341398800, 357127200, 372848400, 388576800, 404902800,
			420026400, 436352400, 452080800, 467802000, 483530400, 499251600, 514980000, 530701200,
			544615200, 562150800, 576064800, 594205200, 607514400, 625654800, 638964000, 657104400,
			671018400, 688554000, 702468000, 720003600, 733917600, 752058000, 765367200, 783507600,
			796816800, 814957200, 828871200, 846406800, 860320800, 877856400, 891770400, 909306000,
			923220000, 941360400, 954669600, 972810000, 986119200, 1004259600, 1018173600,
			1035709200, 1049623200, 1067158800, 1081072800, 1099213200, 1112522400, 1130662800,
			1143972000, 1162112400, 1173607200,
		],
		transition_types: &[
			1, 0, 1, 0, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
			1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["PST", "PDT", "PWT", "PPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "PST", offset: 28800 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "PDT", offset: 25200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const PACIFIC_APIA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2445424384, -1861878784, -631110600, 1285498800, 1301752800, 1316872800, 1325239200,
			1333202400, 1348927200, 1365256800, 1380376800, 1396706400, 1411826400, 1428156000,
			1443276000, 1459605600, 1474725600, 1491055200, 1506175200, 1522504800, 1538229600,
			1554559200, 1569679200, 1586008800, 1601128800, 1617458400,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 45184, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -41216, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -41400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 10 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 14 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 50400, is_dst: true, idx: 22 },
		],
		time_zone_designations: &["LMT", "LMT", "-1130", "-10", "-11", "+13", "+14"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+13", offset: -46800 },
		dst_info: None,
	},
};
const PACIFIC_BOUGAINVILLE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2840178136, -2366790512, -868010400, -768906000, 1419696000],
		transition_types: &[1, 2, 3, 2, 4],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 37336, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 35312, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 17 },
		],
		time_zone_designations: &["LMT", "PMMT", "+10", "+09", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const PACIFIC_EFATE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1829387596, 125409600, 133876800, 433256400, 448977600, 464706000, 480427200,
			496760400, 511876800, 528210000, 543931200, 559659600, 575380800, 591109200, 606830400,
			622558800, 638280000, 654008400, 669729600, 686062800, 696340800, 719931600, 727790400,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 40396, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+12", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const PACIFIC_ENDERBURY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1020470400, 307627200, 788871600],
		transition_types: &[1, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -43200, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: false, idx: 12 },
		],
		time_zone_designations: &["-00", "-12", "-11", "+13"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+13", offset: -46800 },
		dst_info: None,
	},
};
const PACIFIC_FAKAOFO: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2177411704, 1325242800],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -41096, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-11", "+13"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+13", offset: -46800 },
		dst_info: None,
	},
};
const PACIFIC_FIJI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1709985344, 909842400, 920124000, 941896800, 951573600, 1259416800, 1269698400,
			1287842400, 1299333600, 1319292000, 1327154400, 1350741600, 1358604000, 1382796000,
			1390050000, 1414850400, 1421503200, 1446300000, 1452952800, 1478354400, 1484402400,
			1509804000, 1515852000, 1541253600, 1547301600, 1573308000, 1578751200, 1608386400,
			1610805600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 42944, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+13", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const PACIFIC_GALAPAGOS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1230746496, 504939600, 722930400, 728888400],
		transition_types: &[1, 3, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21504, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "-05", "-05", "-06"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-06", offset: 21600 },
		dst_info: None,
	},
};
const PACIFIC_GAMBIER: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1806678012],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -32388, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-09", offset: 32400 },
		dst_info: None,
	},
};
const PACIFIC_KIRITIMATI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2177415040, 307622400, 788868000],
		transition_types: &[1, 2, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -37760, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -38400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 50400, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "-1040", "-10", "+14"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+14", offset: -50400 },
		dst_info: None,
	},
};
const PACIFIC_KOSRAE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3944631116, -2177491916, -1743678000, -1606813200, -1041418800, -907408800,
			-770634000, -7988400, 915105600,
		],
		transition_types: &[1, 2, 3, 2, 4, 3, 2, 5, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -47284, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39116, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "LMT", "+11", "+09", "+10", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const PACIFIC_MARQUESAS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1806676920],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -33480, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -34200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-0930"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-0930", offset: 34200 },
		dst_info: None,
	},
};
const PACIFIC_NAURU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1545131260, -862918200, -767350800, 287418600],
		transition_types: &[1, 2, 1, 3],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 40060, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 41400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 14 },
		],
		time_zone_designations: &["LMT", "+1130", "+09", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const PACIFIC_NIUE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-543069620, -173623200],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -40780, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -40800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 10 },
		],
		time_zone_designations: &["LMT", "-1120", "-11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-11", offset: 39600 },
		dst_info: None,
	},
};
const PACIFIC_NORFOLK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2177493112, -599656320, 152029800, 162916200, 1443882600, 1570287600],
		transition_types: &[1, 2, 3, 2, 4, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 40312, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 40320, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 41400, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 45000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 22 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 26 },
		],
		time_zone_designations: &["LMT", "+1112", "+1130", "+1230", "+11", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "+12", offset: -43200 },
			start_date: TransitionDate { day: MonthWeekDay(10, 1, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(4, 1, 0), time: 10800 },
		}),
	},
};
const PACIFIC_NOUMEA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1829387148, 250002000, 257342400, 281451600, 288878400, 849366000, 857228400,
		],
		transition_types: &[2, 1, 2, 1, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 39948, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 8 },
		],
		time_zone_designations: &["LMT", "+12", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const PACIFIC_PALAU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-3944624276, -2177485076],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -54124, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32276, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "LMT", "+09"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+09", offset: -32400 },
		dst_info: None,
	},
};
const PACIFIC_PITCAIRN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2177421580, 893665800],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -31220, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -30600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 10 },
		],
		time_zone_designations: &["LMT", "-0830", "-08"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-08", offset: 28800 },
		dst_info: None,
	},
};
const PACIFIC_PONAPE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1806748788],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 38388, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "+11"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+11", offset: -39600 },
		dst_info: None,
	},
};
const PACIFIC_RAROTONGA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2209555256, -543072056, 279714600, 289387800, 309952800, 320837400, 341402400,
			352287000, 372852000, 384341400, 404906400, 415791000, 436356000, 447240600, 467805600,
			478690200, 499255200, 510139800, 530704800, 541589400, 562154400, 573643800, 594208800,
			605093400, 625658400, 636543000, 657108000, 667992600,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 48056, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -38344, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -37800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: -34200, is_dst: true, idx: 14 },
		],
		time_zone_designations: &["LMT", "LMT", "-1030", "-10", "-0930"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-10", offset: 36000 },
		dst_info: None,
	},
};
const PACIFIC_SAIPAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3944626740, -2177487540, -885549600, -802256400, -331891200, -281610000, -73728000,
			-29415540, -16704000, -10659600, 9907200, 21394800, 41356800, 52844400, 124819200,
			130863600, 201888000, 209487660, 230659200, 241542000, 977493600,
		],
		transition_types: &[1, 2, 3, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -51660, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 34740, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 39600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 16 },
		],
		time_zone_designations: &["LMT", "LMT", "GST", "+09", "GDT", "ChST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "ChST", offset: -36000 },
		dst_info: None,
	},
};
const PACIFIC_TAHITI: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-1806674504],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -35896, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "-10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "-10", offset: 36000 },
		dst_info: None,
	},
};
const PACIFIC_TONGATAPU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-767189952, -284041200, 939214800, 953384400, 973342800, 980596800, 1004792400,
			1012046400, 1478350800, 1484398800,
		],
		transition_types: &[1, 2, 3, 2, 3, 2, 3, 2, 3, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 44352, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 44400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 46800, is_dst: false, idx: 10 },
			LocalTimeTypeRecord { utc_offset: 50400, is_dst: true, idx: 14 },
		],
		time_zone_designations: &["LMT", "+1220", "+13", "+14"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+13", offset: -46800 },
		dst_info: None,
	},
};
const PACIFIC_WALLIS: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2177494324],
		transition_types: &[1],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 41524, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 43200, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "+12"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+12", offset: -43200 },
		dst_info: None,
	},
};
const PACIFIC_YAP: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2840176120, -2366790512],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 35320, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 35312, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: false, idx: 9 },
		],
		time_zone_designations: &["LMT", "PMMT", "+10"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+10", offset: -36000 },
		dst_info: None,
	},
};
const POLAND: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840145840, -1717032240, -1693706400, -1680483600, -1663455600, -1650150000,
			-1632006000, -1618700400, -1600473600, -1587168000, -1501725600, -931734000,
			-857257200, -844556400, -828226800, -812502000, -796608000, -778726800, -762660000,
			-748486800, -733273200, -715215600, -701910000, -684975600, -670460400, -654130800,
			-639010800, -397094400, -386812800, -371088000, -355363200, -334195200, -323308800,
			-307584000, -291859200, -271296000, -260409600, -239846400, -228960000, -208396800,
			-197510400, -176342400, -166060800, 228873600, 243993600, 260323200, 276048000,
			291772800, 307497600, 323827200, 338947200, 354672000, 370396800, 386121600, 401846400,
			417571200, 433296000, 449020800, 465350400, 481075200, 496800000, 512524800, 528249600,
			543974400, 559699200, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400,
			670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800,
			796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 5, 4, 5, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 5040, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 5040, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 22 },
		],
		time_zone_designations: &["LMT", "WMT", "CEST", "CET", "EEST", "EET"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CET", offset: -3600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CEST", offset: -7200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 10800 },
		}),
	},
};
const PORTUGAL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2713908195, -1830384000, -1689555600, -1677801600, -1667437200, -1647738000,
			-1635814800, -1616202000, -1604365200, -1584666000, -1572742800, -1553043600,
			-1541206800, -1521507600, -1442451600, -1426813200, -1379293200, -1364778000,
			-1348448400, -1333328400, -1316394000, -1301274000, -1284339600, -1269824400,
			-1221440400, -1206925200, -1191200400, -1175475600, -1127696400, -1111971600,
			-1096851600, -1080522000, -1063587600, -1049072400, -1033347600, -1017622800,
			-1002502800, -986173200, -969238800, -950490000, -942022800, -922669200, -906944400,
			-891133200, -877309200, -873684000, -864007200, -857955600, -845859600, -842839200,
			-831348000, -825901200, -814410000, -810784800, -799898400, -794451600, -782960400,
			-779335200, -768448800, -763002000, -749091600, -733366800, -717631200, -701906400,
			-686181600, -670456800, -654732000, -639007200, -623282400, -607557600, -591832800,
			-575503200, -559778400, -544053600, -528328800, -512604000, -496879200, -481154400,
			-465429600, -449704800, -433980000, -417650400, -401925600, -386200800, -370476000,
			-354751200, -339026400, -323301600, -307576800, -291852000, -276127200, -260402400,
			-244677600, -228348000, -212623200, -196898400, -181173600, -165448800, -149724000,
			-133999200, -118274400, 212544000, 228268800, 243993600, 260323200, 276048000,
			291772800, 307501200, 323222400, 338950800, 354675600, 370400400, 386125200, 401850000,
			417578400, 433299600, 449024400, 465354000, 481078800, 496803600, 512528400, 528253200,
			543978000, 559702800, 575427600, 591152400, 606877200, 622602000, 638326800, 654656400,
			670381200, 686106000, 701830800, 717555600, 733280400, 749005200, 764730000, 780454800,
			796179600, 811904400, 828234000,
		],
		transition_types: &[
			0, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 2, 1, 3, 1, 2, 1, 3, 1, 2, 1, 3,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 4, 5, 4, 5, 4, 5, 4, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -2205, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 9 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: true, idx: 22 },
		],
		time_zone_designations: &["LMT", "WEST", "WET", "WEMT", "CET", "CEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WET", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "WEST", offset: -3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const ROC: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2335248360, -1017820800, -766224000, -745833600, -733827600, -716889600, -699613200,
			-683884800, -670669200, -652348800, -639133200, -620812800, -607597200, -589276800,
			-576061200, -562924800, -541760400, -528710400, -510224400, -497174400, -478688400,
			-465638400, -449830800, -434016000, -418208400, -402480000, -386672400, -370944000,
			-355136400, -339408000, -323600400, -302515200, -291978000, -270979200, -260442000,
			133977600, 149785200, 165513600, 181321200, 299606400, 307551600,
		],
		transition_types: &[
			1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
			3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 29160, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "CST", "JST", "CDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: -28800 },
		dst_info: None,
	},
};
const ROK: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-1948782472, -1830414600, -767350800, -681210000, -672228000, -654771600, -640864800,
			-623408400, -609415200, -588848400, -577965600, -498128400, -462702600, -451733400,
			-429784200, -418296600, -399544200, -387451800, -368094600, -356002200, -336645000,
			-324552600, -305195400, -293103000, -264933000, 547578000, 560883600, 579027600,
			592333200,
		],
		transition_types: &[
			1, 2, 4, 3, 4, 3, 4, 3, 4, 3, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 4, 3, 4, 3, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 30472, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 30600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 36000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 34200, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "KST", "JST", "KDT", "KST", "KDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "KST", offset: -32400 },
		dst_info: None,
	},
};
const SINGAPORE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2177477725, -2038200925, -1167634800, -1073028000, -894180000, -879665400, -767005200,
			378662400,
		],
		transition_types: &[1, 2, 3, 4, 5, 6, 5, 7],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 24925, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 24925, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 26400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 26400, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 27000, is_dst: false, idx: 18 },
			LocalTimeTypeRecord { utc_offset: 32400, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: 28800, is_dst: false, idx: 28 },
		],
		time_zone_designations: &["LMT", "SMT", "+07", "+0720", "+0720", "+0730", "+09", "+08"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+08", offset: -28800 },
		dst_info: None,
	},
};
const TURKEY: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840147752, -1869875816, -1693706400, -1680490800, -1570413600, -1552186800,
			-1538359200, -1522551600, -1507514400, -1490583600, -1440208800, -1428030000,
			-1409709600, -1396494000, -931053600, -922676400, -917834400, -892436400, -875844000,
			-764737200, -744343200, -733806000, -716436000, -701924400, -684986400, -670474800,
			-654141600, -639025200, -622087200, -606970800, -590032800, -575521200, -235620000,
			-194842800, -177732000, -165726000, 107910000, 121215600, 133920000, 152665200,
			164678400, 184114800, 196214400, 215564400, 228873600, 245804400, 260323200, 267915600,
			428454000, 433893600, 468111600, 482799600, 496710000, 512521200, 528246000, 543970800,
			559695600, 575420400, 591145200, 606870000, 622594800, 638319600, 654649200, 670374000,
			686098800, 701823600, 717548400, 733273200, 748998000, 764118000, 780447600, 796172400,
			811897200, 828226800, 846370800, 859676400, 877820400, 891126000, 909270000, 922575600,
			941324400, 954025200, 972774000, 985474800, 1004223600, 1017529200, 1035673200,
			1048978800, 1067122800, 1080428400, 1099177200, 1111878000, 1130626800, 1143327600,
			1162076400, 1174784400, 1193533200, 1206838800, 1224982800, 1238288400, 1256432400,
			1269738000, 1288486800, 1301274000, 1319936400, 1332637200, 1351386000, 1364691600,
			1382835600, 1396227600, 1414285200, 1427590800, 1446944400, 1459040400, 1473195600,
		],
		transition_types: &[
			1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4, 5, 4, 3, 2, 3, 2, 3, 2, 3, 2,
			3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
			2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 4,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 6952, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 7016, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 13 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 21 },
		],
		time_zone_designations: &["LMT", "IMT", "EEST", "EET", "+03", "+04"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "+03", offset: -10800 },
		dst_info: None,
	},
};
const US_ALASKA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188951224, -880200000, -769395600, -765378000, -86882400, -21470400,
			-5749200, 9979200, 25700400, 41428800, 57754800, 73483200, 89204400, 104932800,
			120654000, 126705600, 152103600, 162388800, 183553200, 199281600, 215607600, 230731200,
			247057200, 262785600, 278506800, 294235200, 309956400, 325684800, 341406000, 357134400,
			372855600, 388584000, 404910000, 420033600, 436359600, 439030800, 452084400, 467805600,
			483534000, 499255200, 514983600, 530704800, 544618800, 562154400, 576068400, 594208800,
			607518000, 625658400, 638967600, 657108000, 671022000, 688557600, 702471600, 720007200,
			733921200, 752061600, 765370800, 783511200, 796820400, 814960800, 828874800, 846410400,
			860324400, 877860000, 891774000, 909309600, 923223600, 941364000, 954673200, 972813600,
			986122800, 1004263200, 1018177200, 1035712800, 1049626800, 1067162400, 1081076400,
			1099216800, 1112526000, 1130666400, 1143975600, 1162116000, 1173610800,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 7, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
			9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 50424, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -35976, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: true, idx: 21 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 26 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: true, idx: 30 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: false, idx: 35 },
		],
		time_zone_designations: &[
			"LMT", "LMT", "AST", "AWT", "APT", "AHST", "AHDT", "YST", "AKDT", "AKST",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "AKST", offset: 32400 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "AKDT", offset: 28800 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_ALEUTIAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-3225223727, -2188944802, -880196400, -769395600, -765374400, -86878800, -21466800,
			-5745600, 9982800, 25704000, 41432400, 57758400, 73486800, 89208000, 104936400,
			120657600, 126709200, 152107200, 162392400, 183556800, 199285200, 215611200, 230734800,
			247060800, 262789200, 278510400, 294238800, 309960000, 325688400, 341409600, 357138000,
			372859200, 388587600, 404913600, 420037200, 436363200, 439034400, 452088000, 467809200,
			483537600, 499258800, 514987200, 530708400, 544622400, 562158000, 576072000, 594212400,
			607521600, 625662000, 638971200, 657111600, 671025600, 688561200, 702475200, 720010800,
			733924800, 752065200, 765374400, 783514800, 796824000, 814964400, 828878400, 846414000,
			860328000, 877863600, 891777600, 909313200, 923227200, 941367600, 954676800, 972817200,
			986126400, 1004266800, 1018180800, 1035716400, 1049630400, 1067166000, 1081080000,
			1099220400, 1112529600, 1130670000, 1143979200, 1162119600, 1173614400,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 7, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
			9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8, 9, 8,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 44002, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -42398, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: true, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 24 },
			LocalTimeTypeRecord { utc_offset: -32400, is_dst: true, idx: 29 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 25 },
		],
		time_zone_designations: &[
			"LMT", "LMT", "NST", "NWT", "NPT", "BST", "BDT", "AHST", "HDT", "HST",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "HST", offset: 36000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "HDT", offset: 32400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_ARIZONA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717643600, -1633273200, -1615132800, -1601823600, -1583683200, -880210800,
			-820519140, -812653140, -796845540, -84380400, -68659200,
		],
		transition_types: &[2, 1, 2, 1, 2, 3, 2, 3, 2, 1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -26898, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: None,
	},
};
const US_CENTRAL: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -1563724800,
			-1551632400, -1538928000, -1520182800, -1504454400, -1491757200, -1473004800,
			-1459702800, -1441555200, -1428253200, -1410105600, -1396803600, -1378656000,
			-1365354000, -1347206400, -1333904400, -1315152000, -1301850000, -1283702400,
			-1270400400, -1252252800, -1238950800, -1220803200, -1207501200, -1189353600,
			-1176051600, -1157299200, -1144602000, -1125849600, -1112547600, -1094400000,
			-1081098000, -1067788800, -1045414800, -1031500800, -1018198800, -1000051200,
			-986749200, -967996800, -955299600, -936547200, -923245200, -905097600, -891795600,
			-880214400, -769395600, -765392400, -747244800, -733942800, -715795200, -702493200,
			-684345600, -671043600, -652896000, -639594000, -620841600, -608144400, -589392000,
			-576090000, -557942400, -544640400, -526492800, -513190800, -495043200, -481741200,
			-463593600, -447267600, -431539200, -415818000, -400089600, -384368400, -368640000,
			-352918800, -337190400, -321469200, -305740800, -289414800, -273686400, -257965200,
			-242236800, -226515600, -210787200, -195066000, -179337600, -163616400, -147888000,
			-131562000, -116438400, -100112400, -84384000, -68662800, -52934400, -37213200,
			-21484800, -5763600, 9964800, 25686000, 41414400, 57740400, 73468800, 89190000,
			104918400, 120639600, 126691200, 152089200, 162374400, 183538800, 199267200, 215593200,
			230716800, 247042800, 262771200, 278492400, 294220800, 309942000, 325670400, 341391600,
			357120000, 372841200, 388569600, 404895600, 420019200, 436345200, 452073600, 467794800,
			483523200, 499244400, 514972800, 530694000, 544608000, 562143600, 576057600, 594198000,
			607507200, 625647600, 638956800, 657097200, 671011200, 688546800, 702460800, 719996400,
			733910400, 752050800, 765360000, 783500400, 796809600, 814950000, 828864000, 846399600,
			860313600, 877849200, 891763200, 909298800, 923212800, 941353200, 954662400, 972802800,
			986112000, 1004252400, 1018166400, 1035702000, 1049616000, 1067151600, 1081065600,
			1099206000, 1112515200, 1130655600, 1143964800, 1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 4, 5, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -21036, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "EST", "CWT", "CPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_EAST_INDIANA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -900259200,
			-891795600, -880214400, -769395600, -765392400, -747244800, -733942800, -715795200,
			-702493200, -684345600, -671043600, -652896000, -639594000, -620841600, -608144400,
			-589392000, -576090000, -557942400, -544640400, -526492800, -513190800, -495043200,
			-481741200, -463593600, -386787600, -368640000, -21488400, -5767200, 9961200, 25682400,
			1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5,
			2, 5, 6, 5, 6, 5, 6, 5, 6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20678, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 24 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_EASTERN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717650800, -1633280400, -1615140000, -1601830800, -1583690400, -1570381200,
			-1551636000, -1536512400, -1523210400, -1504458000, -1491760800, -1473008400,
			-1459706400, -1441558800, -1428256800, -1410109200, -1396807200, -1378659600,
			-1365357600, -1347210000, -1333908000, -1315155600, -1301853600, -1283706000,
			-1270404000, -1252256400, -1238954400, -1220806800, -1207504800, -1189357200,
			-1176055200, -1157302800, -1144605600, -1125853200, -1112551200, -1094403600,
			-1081101600, -1062954000, -1049652000, -1031504400, -1018202400, -1000054800,
			-986752800, -968000400, -955303200, -936550800, -923248800, -905101200, -891799200,
			-880218000, -769395600, -765396000, -747248400, -733946400, -715798800, -702496800,
			-684349200, -671047200, -652899600, -639597600, -620845200, -608148000, -589395600,
			-576093600, -557946000, -544644000, -526496400, -513194400, -495046800, -481744800,
			-463597200, -447271200, -431542800, -415821600, -400093200, -384372000, -368643600,
			-352922400, -337194000, -321472800, -305744400, -289418400, -273690000, -257968800,
			-242240400, -226519200, -210790800, -195069600, -179341200, -163620000, -147891600,
			-131565600, -116442000, -100116000, -84387600, -68666400, -52938000, -37216800,
			-21488400, -5767200, 9961200, 25682400, 41410800, 57736800, 73465200, 89186400,
			104914800, 120636000, 126687600, 152085600, 162370800, 183535200, 199263600, 215589600,
			230713200, 247039200, 262767600, 278488800, 294217200, 309938400, 325666800, 341388000,
			357116400, 372837600, 388566000, 404892000, 420015600, 436341600, 452070000, 467791200,
			483519600, 499240800, 514969200, 530690400, 544604400, 562140000, 576054000, 594194400,
			607503600, 625644000, 638953200, 657093600, 671007600, 688543200, 702457200, 719992800,
			733906800, 752047200, 765356400, 783496800, 796806000, 814946400, 828860400, 846396000,
			860310000, 877845600, 891759600, 909295200, 923209200, 941349600, 954658800, 972799200,
			986108400, 1004248800, 1018162800, 1035698400, 1049612400, 1067148000, 1081062000,
			1099202400, 1112511600, 1130652000, 1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -17762, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "EDT", "EST", "EWT", "EPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_HAWAII: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2334101314, -1157283000, -1155436200, -880198200, -769395600, -765376200, -712150200,
		],
		transition_types: &[1, 2, 1, 3, 4, 1, 5],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -37886, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -37800, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -34200, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -34200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -34200, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -36000, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "HST", "HDT", "HWT", "HPT", "HST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "HST", offset: 36000 },
		dst_info: None,
	},
};
const US_INDIANA_STARKE: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717647200, -1633276800, -1615136400, -1601827200, -1583686800, -880214400,
			-769395600, -765392400, -715795200, -702493200, -684345600, -671043600, -652896000,
			-639594000, -620841600, -608144400, -589392000, -576090000, -557942400, -544640400,
			-526492800, -513190800, -495043200, -481741200, -463593600, -447267600, -431539200,
			-415818000, -400089600, -386787600, -368640000, -355338000, -337190400, -321469200,
			-305740800, -289414800, -273686400, -257965200, -242236800, -195066000, -84384000,
			-68662800, -52934400, -37213200, -21484800, -5763600, 9964800, 25686000, 41414400,
			57740400, 73468800, 89190000, 104918400, 120639600, 126691200, 152089200, 162374400,
			183538800, 199267200, 215593200, 230716800, 247042800, 262771200, 278492400, 294220800,
			309942000, 325670400, 341391600, 357120000, 372841200, 388569600, 404895600, 420019200,
			436345200, 452073600, 467794800, 483523200, 499244400, 514972800, 530694000, 544608000,
			562143600, 576057600, 594198000, 607507200, 625647600, 638956800, 657097200, 671011200,
			688546800, 1143961200, 1162105200, 1173600000,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 5, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 5, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -20790, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 20 },
		],
		time_zone_designations: &["LMT", "CDT", "CST", "CWT", "CPT", "EST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "CST", offset: 21600 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "CDT", offset: 18000 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_MICHIGAN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2051202469, -1724083200, -880218000, -769395600, -765396000, -684349200, -671047200,
			-80506740, -68666400, -52938000, -37216800, 104914800, 120636000, 126687600, 152085600,
			167814000, 183535200, 199263600, 215589600, 230713200, 247039200, 262767600, 278488800,
			294217200, 309938400, 325666800, 341388000, 357116400, 372837600, 388566000, 404892000,
			420015600, 436341600, 452070000, 467791200, 483519600, 499240800, 514969200, 530690400,
			544604400, 562140000, 576054000, 594194400, 607503600, 625644000, 638953200, 657093600,
			671007600, 688543200, 702457200, 719992800, 733906800, 752047200, 765356400, 783496800,
			796806000, 814946400, 828860400, 846396000, 860310000, 877845600, 891759600, 909295200,
			923209200, 941349600, 954658800, 972799200, 986108400, 1004248800, 1018162800,
			1035698400, 1049612400, 1067148000, 1081062000, 1099202400, 1112511600, 1130652000,
			1143961200, 1162101600, 1173596400,
		],
		transition_types: &[
			1, 2, 3, 4, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2,
			5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5,
			2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5, 2, 5,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -19931, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -18000, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 16 },
			LocalTimeTypeRecord { utc_offset: -14400, is_dst: true, idx: 20 },
		],
		time_zone_designations: &["LMT", "CST", "EST", "EWT", "EPT", "EDT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "EST", offset: 18000 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "EDT", offset: 14400 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_MOUNTAIN: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717643600, -1633273200, -1615132800, -1601823600, -1583683200, -1570374000,
			-1551628800, -1538924400, -1534089600, -880210800, -769395600, -765388800, -147884400,
			-131558400, -116434800, -100108800, -84380400, -68659200, -52930800, -37209600,
			-21481200, -5760000, 9968400, 25689600, 41418000, 57744000, 73472400, 89193600,
			104922000, 120643200, 126694800, 152092800, 162378000, 183542400, 199270800, 215596800,
			230720400, 247046400, 262774800, 278496000, 294224400, 309945600, 325674000, 341395200,
			357123600, 372844800, 388573200, 404899200, 420022800, 436348800, 452077200, 467798400,
			483526800, 499248000, 514976400, 530697600, 544611600, 562147200, 576061200, 594201600,
			607510800, 625651200, 638960400, 657100800, 671014800, 688550400, 702464400, 720000000,
			733914000, 752054400, 765363600, 783504000, 796813200, 814953600, 828867600, 846403200,
			860317200, 877852800, 891766800, 909302400, 923216400, 941356800, 954666000, 972806400,
			986115600, 1004256000, 1018170000, 1035705600, 1049619600, 1067155200, 1081069200,
			1099209600, 1112518800, 1130659200, 1143968400, 1162108800, 1173603600,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -25196, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -21600, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "MDT", "MST", "MWT", "MPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MST", offset: 25200 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "MDT", offset: 21600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_PACIFIC: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2717640000, -1633269600, -1615129200, -1601820000, -1583679600, -880207200,
			-769395600, -765385200, -687967140, -662655600, -620838000, -608137200, -589388400,
			-576082800, -557938800, -544633200, -526489200, -513183600, -495039600, -481734000,
			-463590000, -450284400, -431535600, -418230000, -400086000, -386780400, -368636400,
			-355330800, -337186800, -323881200, -305737200, -292431600, -273682800, -260982000,
			-242233200, -226508400, -210783600, -195058800, -179334000, -163609200, -147884400,
			-131554800, -116434800, -100105200, -84376800, -68655600, -52927200, -37206000,
			-21477600, -5756400, 9972000, 25693200, 41421600, 57747600, 73476000, 89197200,
			104925600, 120646800, 126698400, 152096400, 162381600, 183546000, 199274400, 215600400,
			230724000, 247050000, 262778400, 278499600, 294228000, 309949200, 325677600, 341398800,
			357127200, 372848400, 388576800, 404902800, 420026400, 436352400, 452080800, 467802000,
			483530400, 499251600, 514980000, 530701200, 544615200, 562150800, 576064800, 594205200,
			607514400, 625654800, 638964000, 657104400, 671018400, 688554000, 702468000, 720003600,
			733917600, 752058000, 765367200, 783507600, 796816800, 814957200, 828871200, 846406800,
			860320800, 877856400, 891770400, 909306000, 923220000, 941360400, 954669600, 972810000,
			986119200, 1004259600, 1018173600, 1035709200, 1049623200, 1067158800, 1081072800,
			1099213200, 1112522400, 1130662800, 1143972000, 1162112400, 1173607200,
		],
		transition_types: &[
			2, 1, 2, 1, 2, 3, 4, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
			2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
			1, 2, 1, 2, 1, 2, 1, 2, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: -28378, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 4 },
			LocalTimeTypeRecord { utc_offset: -28800, is_dst: false, idx: 8 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: -25200, is_dst: true, idx: 16 },
		],
		time_zone_designations: &["LMT", "PDT", "PST", "PWT", "PPT"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "PST", offset: 28800 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "PDT", offset: 25200 },
			start_date: TransitionDate { day: MonthWeekDay(3, 2, 0), time: 7200 },
			end_date: TransitionDate { day: MonthWeekDay(11, 1, 0), time: 7200 },
		}),
	},
};
const US_SAMOA: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[-2445424632, -1861879032],
		transition_types: &[1, 2],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 45432, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -40968, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: -39600, is_dst: false, idx: 4 },
		],
		time_zone_designations: &["LMT", "LMT", "SST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "SST", offset: 39600 },
		dst_info: None,
	},
};
const W_SU: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			-2840149817, -1688265017, -1656819079, -1641353479, -1627965079, -1618716679,
			-1596429079, -1593820800, -1589860800, -1542427200, -1539493200, -1525323600,
			-1522728000, -1491188400, -1247536800, 354920400, 370728000, 386456400, 402264000,
			417992400, 433800000, 449614800, 465346800, 481071600, 496796400, 512521200, 528246000,
			543970800, 559695600, 575420400, 591145200, 606870000, 622594800, 638319600, 654649200,
			670374000, 686102400, 695779200, 701823600, 717548400, 733273200, 748998000, 764722800,
			780447600, 796172400, 811897200, 828226800, 846370800, 859676400, 877820400, 891126000,
			909270000, 922575600, 941324400, 954025200, 972774000, 985474800, 1004223600,
			1017529200, 1035673200, 1048978800, 1067122800, 1080428400, 1099177200, 1111878000,
			1130626800, 1143327600, 1162076400, 1174777200, 1193526000, 1206831600, 1224975600,
			1238281200, 1256425200, 1269730800, 1288479600, 1301180400, 1414274400,
		],
		transition_types: &[
			1, 3, 2, 3, 4, 2, 4, 5, 6, 5, 7, 5, 6, 8, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 9, 8, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6,
			5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 10, 6,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 9017, is_dst: false, idx: 0 },
			LocalTimeTypeRecord { utc_offset: 9017, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 12679, is_dst: true, idx: 8 },
			LocalTimeTypeRecord { utc_offset: 9079, is_dst: false, idx: 4 },
			LocalTimeTypeRecord { utc_offset: 16279, is_dst: true, idx: 12 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: true, idx: 17 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: false, idx: 21 },
			LocalTimeTypeRecord { utc_offset: 18000, is_dst: true, idx: 25 },
			LocalTimeTypeRecord { utc_offset: 7200, is_dst: false, idx: 29 },
			LocalTimeTypeRecord { utc_offset: 10800, is_dst: true, idx: 33 },
			LocalTimeTypeRecord { utc_offset: 14400, is_dst: false, idx: 21 },
		],
		time_zone_designations: &[
			"LMT", "MMT", "MST", "MMT", "MDST", "MSD", "MSK", "+05", "EET", "EEST", "MSK",
		],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "MSK", offset: -10800 },
		dst_info: None,
	},
};
const WET: TzifData = TzifData {
	data_block: DataBlock {
		transition_times: &[
			228877200, 243997200, 260326800, 276051600, 291776400, 307501200, 323830800, 338950800,
			354675600, 370400400, 386125200, 401850000, 417574800, 433299600, 449024400, 465354000,
			481078800, 496803600, 512528400, 528253200, 543978000, 559702800, 575427600, 591152400,
			606877200, 622602000, 638326800, 654656400, 670381200, 686106000, 701830800, 717555600,
			733280400, 749005200, 764730000, 780454800, 796179600, 811904400, 828234000,
		],
		transition_types: &[
			1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
			0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
		],
		local_time_type_records: &[
			LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 5 },
			LocalTimeTypeRecord { utc_offset: 3600, is_dst: true, idx: 0 },
		],
		time_zone_designations: &["WET", "WEST"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString {
		std_info: ZoneVariantInfo { name: "WET", offset: 0 },
		dst_info: Some(DstTransitionInfo {
			variant_info: ZoneVariantInfo { name: "WEST", offset: -3600 },
			start_date: TransitionDate { day: MonthWeekDay(3, 5, 0), time: 3600 },
			end_date: TransitionDate { day: MonthWeekDay(10, 5, 0), time: 7200 },
		}),
	},
};
const ZULU: TzifData = TzifData {
	data_block: DataBlock {
		local_time_type_records: &[LocalTimeTypeRecord { utc_offset: 0, is_dst: false, idx: 0 }],
		time_zone_designations: &["UTC"],
		..DataBlock::EMPTY
	},
	footer: PosixTzString { std_info: ZoneVariantInfo { name: "UTC", offset: 0 }, dst_info: None },
};
/// Get a time zone's data from its name. Returns `None` if the name is not recognized.
fn zone_data_from_name(s: &str) -> Option<TzifData> {
	if s.eq_ignore_ascii_case("Africa/Abidjan") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Accra") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Addis_Ababa") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Algiers") {
		Some(AFRICA_ALGIERS)
	} else if s.eq_ignore_ascii_case("Africa/Asmara") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Asmera") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Bamako") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Bangui") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Banjul") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Bissau") {
		Some(AFRICA_BISSAU)
	} else if s.eq_ignore_ascii_case("Africa/Blantyre") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Brazzaville") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Bujumbura") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Cairo") {
		Some(EGYPT)
	} else if s.eq_ignore_ascii_case("Africa/Casablanca") {
		Some(AFRICA_CASABLANCA)
	} else if s.eq_ignore_ascii_case("Africa/Ceuta") {
		Some(AFRICA_CEUTA)
	} else if s.eq_ignore_ascii_case("Africa/Conakry") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Dakar") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Dar_es_Salaam") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Djibouti") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Douala") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/El_Aaiun") {
		Some(AFRICA_EL_AAIUN)
	} else if s.eq_ignore_ascii_case("Africa/Freetown") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Gaborone") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Harare") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Johannesburg") {
		Some(AFRICA_MBABANE)
	} else if s.eq_ignore_ascii_case("Africa/Juba") {
		Some(AFRICA_JUBA)
	} else if s.eq_ignore_ascii_case("Africa/Kampala") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Khartoum") {
		Some(AFRICA_KHARTOUM)
	} else if s.eq_ignore_ascii_case("Africa/Kigali") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Kinshasa") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Lagos") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Libreville") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Lome") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Luanda") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Lubumbashi") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Lusaka") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Malabo") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Maputo") {
		Some(AFRICA_LUSAKA)
	} else if s.eq_ignore_ascii_case("Africa/Maseru") {
		Some(AFRICA_MBABANE)
	} else if s.eq_ignore_ascii_case("Africa/Mbabane") {
		Some(AFRICA_MBABANE)
	} else if s.eq_ignore_ascii_case("Africa/Mogadishu") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Monrovia") {
		Some(AFRICA_MONROVIA)
	} else if s.eq_ignore_ascii_case("Africa/Nairobi") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Africa/Ndjamena") {
		Some(AFRICA_NDJAMENA)
	} else if s.eq_ignore_ascii_case("Africa/Niamey") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Nouakchott") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Ouagadougou") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Porto-Novo") {
		Some(AFRICA_PORTO_NOVO)
	} else if s.eq_ignore_ascii_case("Africa/Sao_Tome") {
		Some(AFRICA_SAO_TOME)
	} else if s.eq_ignore_ascii_case("Africa/Timbuktu") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Africa/Tripoli") {
		Some(LIBYA)
	} else if s.eq_ignore_ascii_case("Africa/Tunis") {
		Some(AFRICA_TUNIS)
	} else if s.eq_ignore_ascii_case("Africa/Windhoek") {
		Some(AFRICA_WINDHOEK)
	} else if s.eq_ignore_ascii_case("America/Adak") {
		Some(US_ALEUTIAN)
	} else if s.eq_ignore_ascii_case("America/Anchorage") {
		Some(US_ALASKA)
	} else if s.eq_ignore_ascii_case("America/Anguilla") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Antigua") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Araguaina") {
		Some(AMERICA_ARAGUAINA)
	} else if s.eq_ignore_ascii_case("America/Argentina/Buenos_Aires") {
		Some(AMERICA_BUENOS_AIRES)
	} else if s.eq_ignore_ascii_case("America/Argentina/Catamarca") {
		Some(AMERICA_CATAMARCA)
	} else if s.eq_ignore_ascii_case("America/Argentina/ComodRivadavia") {
		Some(AMERICA_CATAMARCA)
	} else if s.eq_ignore_ascii_case("America/Argentina/Cordoba") {
		Some(AMERICA_ROSARIO)
	} else if s.eq_ignore_ascii_case("America/Argentina/Jujuy") {
		Some(AMERICA_JUJUY)
	} else if s.eq_ignore_ascii_case("America/Argentina/La_Rioja") {
		Some(AMERICA_ARGENTINA_LA_RIOJA)
	} else if s.eq_ignore_ascii_case("America/Argentina/Mendoza") {
		Some(AMERICA_MENDOZA)
	} else if s.eq_ignore_ascii_case("America/Argentina/Rio_Gallegos") {
		Some(AMERICA_ARGENTINA_RIO_GALLEGOS)
	} else if s.eq_ignore_ascii_case("America/Argentina/Salta") {
		Some(AMERICA_ARGENTINA_SALTA)
	} else if s.eq_ignore_ascii_case("America/Argentina/San_Juan") {
		Some(AMERICA_ARGENTINA_SAN_JUAN)
	} else if s.eq_ignore_ascii_case("America/Argentina/San_Luis") {
		Some(AMERICA_ARGENTINA_SAN_LUIS)
	} else if s.eq_ignore_ascii_case("America/Argentina/Tucuman") {
		Some(AMERICA_ARGENTINA_TUCUMAN)
	} else if s.eq_ignore_ascii_case("America/Argentina/Ushuaia") {
		Some(AMERICA_ARGENTINA_USHUAIA)
	} else if s.eq_ignore_ascii_case("America/Aruba") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Asuncion") {
		Some(AMERICA_ASUNCION)
	} else if s.eq_ignore_ascii_case("America/Atikokan") {
		Some(AMERICA_CORAL_HARBOUR)
	} else if s.eq_ignore_ascii_case("America/Atka") {
		Some(US_ALEUTIAN)
	} else if s.eq_ignore_ascii_case("America/Bahia") {
		Some(AMERICA_BAHIA)
	} else if s.eq_ignore_ascii_case("America/Bahia_Banderas") {
		Some(AMERICA_BAHIA_BANDERAS)
	} else if s.eq_ignore_ascii_case("America/Barbados") {
		Some(AMERICA_BARBADOS)
	} else if s.eq_ignore_ascii_case("America/Belem") {
		Some(AMERICA_BELEM)
	} else if s.eq_ignore_ascii_case("America/Belize") {
		Some(AMERICA_BELIZE)
	} else if s.eq_ignore_ascii_case("America/Blanc-Sablon") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Boa_Vista") {
		Some(AMERICA_BOA_VISTA)
	} else if s.eq_ignore_ascii_case("America/Bogota") {
		Some(AMERICA_BOGOTA)
	} else if s.eq_ignore_ascii_case("America/Boise") {
		Some(AMERICA_BOISE)
	} else if s.eq_ignore_ascii_case("America/Buenos_Aires") {
		Some(AMERICA_BUENOS_AIRES)
	} else if s.eq_ignore_ascii_case("America/Cambridge_Bay") {
		Some(AMERICA_CAMBRIDGE_BAY)
	} else if s.eq_ignore_ascii_case("America/Campo_Grande") {
		Some(AMERICA_CAMPO_GRANDE)
	} else if s.eq_ignore_ascii_case("America/Cancun") {
		Some(AMERICA_CANCUN)
	} else if s.eq_ignore_ascii_case("America/Caracas") {
		Some(AMERICA_CARACAS)
	} else if s.eq_ignore_ascii_case("America/Catamarca") {
		Some(AMERICA_CATAMARCA)
	} else if s.eq_ignore_ascii_case("America/Cayenne") {
		Some(AMERICA_CAYENNE)
	} else if s.eq_ignore_ascii_case("America/Cayman") {
		Some(AMERICA_CORAL_HARBOUR)
	} else if s.eq_ignore_ascii_case("America/Chicago") {
		Some(US_CENTRAL)
	} else if s.eq_ignore_ascii_case("America/Chihuahua") {
		Some(AMERICA_CHIHUAHUA)
	} else if s.eq_ignore_ascii_case("America/Ciudad_Juarez") {
		Some(AMERICA_CIUDAD_JUAREZ)
	} else if s.eq_ignore_ascii_case("America/Coral_Harbour") {
		Some(AMERICA_CORAL_HARBOUR)
	} else if s.eq_ignore_ascii_case("America/Cordoba") {
		Some(AMERICA_ROSARIO)
	} else if s.eq_ignore_ascii_case("America/Costa_Rica") {
		Some(AMERICA_COSTA_RICA)
	} else if s.eq_ignore_ascii_case("America/Creston") {
		Some(US_ARIZONA)
	} else if s.eq_ignore_ascii_case("America/Cuiaba") {
		Some(AMERICA_CUIABA)
	} else if s.eq_ignore_ascii_case("America/Curacao") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Danmarkshavn") {
		Some(AMERICA_DANMARKSHAVN)
	} else if s.eq_ignore_ascii_case("America/Dawson") {
		Some(AMERICA_DAWSON)
	} else if s.eq_ignore_ascii_case("America/Dawson_Creek") {
		Some(AMERICA_DAWSON_CREEK)
	} else if s.eq_ignore_ascii_case("America/Denver") {
		Some(US_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("America/Detroit") {
		Some(US_MICHIGAN)
	} else if s.eq_ignore_ascii_case("America/Dominica") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Edmonton") {
		Some(CANADA_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("America/Eirunepe") {
		Some(AMERICA_EIRUNEPE)
	} else if s.eq_ignore_ascii_case("America/El_Salvador") {
		Some(AMERICA_EL_SALVADOR)
	} else if s.eq_ignore_ascii_case("America/Ensenada") {
		Some(MEXICO_BAJA_NORTE)
	} else if s.eq_ignore_ascii_case("America/Fort_Nelson") {
		Some(AMERICA_FORT_NELSON)
	} else if s.eq_ignore_ascii_case("America/Fort_Wayne") {
		Some(US_EAST_INDIANA)
	} else if s.eq_ignore_ascii_case("America/Fortaleza") {
		Some(AMERICA_FORTALEZA)
	} else if s.eq_ignore_ascii_case("America/Glace_Bay") {
		Some(AMERICA_GLACE_BAY)
	} else if s.eq_ignore_ascii_case("America/Godthab") {
		Some(AMERICA_GODTHAB)
	} else if s.eq_ignore_ascii_case("America/Goose_Bay") {
		Some(AMERICA_GOOSE_BAY)
	} else if s.eq_ignore_ascii_case("America/Grand_Turk") {
		Some(AMERICA_GRAND_TURK)
	} else if s.eq_ignore_ascii_case("America/Grenada") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Guadeloupe") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Guatemala") {
		Some(AMERICA_GUATEMALA)
	} else if s.eq_ignore_ascii_case("America/Guayaquil") {
		Some(AMERICA_GUAYAQUIL)
	} else if s.eq_ignore_ascii_case("America/Guyana") {
		Some(AMERICA_GUYANA)
	} else if s.eq_ignore_ascii_case("America/Halifax") {
		Some(CANADA_ATLANTIC)
	} else if s.eq_ignore_ascii_case("America/Havana") {
		Some(CUBA)
	} else if s.eq_ignore_ascii_case("America/Hermosillo") {
		Some(AMERICA_HERMOSILLO)
	} else if s.eq_ignore_ascii_case("America/Indiana/Indianapolis") {
		Some(US_EAST_INDIANA)
	} else if s.eq_ignore_ascii_case("America/Indiana/Knox") {
		Some(US_INDIANA_STARKE)
	} else if s.eq_ignore_ascii_case("America/Indiana/Marengo") {
		Some(AMERICA_INDIANA_MARENGO)
	} else if s.eq_ignore_ascii_case("America/Indiana/Petersburg") {
		Some(AMERICA_INDIANA_PETERSBURG)
	} else if s.eq_ignore_ascii_case("America/Indiana/Tell_City") {
		Some(AMERICA_INDIANA_TELL_CITY)
	} else if s.eq_ignore_ascii_case("America/Indiana/Vevay") {
		Some(AMERICA_INDIANA_VEVAY)
	} else if s.eq_ignore_ascii_case("America/Indiana/Vincennes") {
		Some(AMERICA_INDIANA_VINCENNES)
	} else if s.eq_ignore_ascii_case("America/Indiana/Winamac") {
		Some(AMERICA_INDIANA_WINAMAC)
	} else if s.eq_ignore_ascii_case("America/Indianapolis") {
		Some(US_EAST_INDIANA)
	} else if s.eq_ignore_ascii_case("America/Inuvik") {
		Some(AMERICA_INUVIK)
	} else if s.eq_ignore_ascii_case("America/Iqaluit") {
		Some(AMERICA_PANGNIRTUNG)
	} else if s.eq_ignore_ascii_case("America/Jamaica") {
		Some(JAMAICA)
	} else if s.eq_ignore_ascii_case("America/Jujuy") {
		Some(AMERICA_JUJUY)
	} else if s.eq_ignore_ascii_case("America/Juneau") {
		Some(AMERICA_JUNEAU)
	} else if s.eq_ignore_ascii_case("America/Kentucky/Louisville") {
		Some(AMERICA_LOUISVILLE)
	} else if s.eq_ignore_ascii_case("America/Kentucky/Monticello") {
		Some(AMERICA_KENTUCKY_MONTICELLO)
	} else if s.eq_ignore_ascii_case("America/Knox_IN") {
		Some(US_INDIANA_STARKE)
	} else if s.eq_ignore_ascii_case("America/Kralendijk") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/La_Paz") {
		Some(AMERICA_LA_PAZ)
	} else if s.eq_ignore_ascii_case("America/Lima") {
		Some(AMERICA_LIMA)
	} else if s.eq_ignore_ascii_case("America/Los_Angeles") {
		Some(US_PACIFIC)
	} else if s.eq_ignore_ascii_case("America/Louisville") {
		Some(AMERICA_LOUISVILLE)
	} else if s.eq_ignore_ascii_case("America/Lower_Princes") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Maceio") {
		Some(AMERICA_MACEIO)
	} else if s.eq_ignore_ascii_case("America/Managua") {
		Some(AMERICA_MANAGUA)
	} else if s.eq_ignore_ascii_case("America/Manaus") {
		Some(BRAZIL_WEST)
	} else if s.eq_ignore_ascii_case("America/Marigot") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Martinique") {
		Some(AMERICA_MARTINIQUE)
	} else if s.eq_ignore_ascii_case("America/Matamoros") {
		Some(AMERICA_MATAMOROS)
	} else if s.eq_ignore_ascii_case("America/Mazatlan") {
		Some(MEXICO_BAJA_SUR)
	} else if s.eq_ignore_ascii_case("America/Mendoza") {
		Some(AMERICA_MENDOZA)
	} else if s.eq_ignore_ascii_case("America/Menominee") {
		Some(AMERICA_MENOMINEE)
	} else if s.eq_ignore_ascii_case("America/Merida") {
		Some(AMERICA_MERIDA)
	} else if s.eq_ignore_ascii_case("America/Metlakatla") {
		Some(AMERICA_METLAKATLA)
	} else if s.eq_ignore_ascii_case("America/Mexico_City") {
		Some(MEXICO_GENERAL)
	} else if s.eq_ignore_ascii_case("America/Miquelon") {
		Some(AMERICA_MIQUELON)
	} else if s.eq_ignore_ascii_case("America/Moncton") {
		Some(AMERICA_MONCTON)
	} else if s.eq_ignore_ascii_case("America/Monterrey") {
		Some(AMERICA_MONTERREY)
	} else if s.eq_ignore_ascii_case("America/Montevideo") {
		Some(AMERICA_MONTEVIDEO)
	} else if s.eq_ignore_ascii_case("America/Montreal") {
		Some(CANADA_EASTERN)
	} else if s.eq_ignore_ascii_case("America/Montserrat") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Nassau") {
		Some(CANADA_EASTERN)
	} else if s.eq_ignore_ascii_case("America/New_York") {
		Some(US_EASTERN)
	} else if s.eq_ignore_ascii_case("America/Nipigon") {
		Some(CANADA_EASTERN)
	} else if s.eq_ignore_ascii_case("America/Nome") {
		Some(AMERICA_NOME)
	} else if s.eq_ignore_ascii_case("America/Noronha") {
		Some(BRAZIL_DE_NORONHA)
	} else if s.eq_ignore_ascii_case("America/North_Dakota/Beulah") {
		Some(AMERICA_NORTH_DAKOTA_BEULAH)
	} else if s.eq_ignore_ascii_case("America/North_Dakota/Center") {
		Some(AMERICA_NORTH_DAKOTA_CENTER)
	} else if s.eq_ignore_ascii_case("America/North_Dakota/New_Salem") {
		Some(AMERICA_NORTH_DAKOTA_NEW_SALEM)
	} else if s.eq_ignore_ascii_case("America/Nuuk") {
		Some(AMERICA_GODTHAB)
	} else if s.eq_ignore_ascii_case("America/Ojinaga") {
		Some(AMERICA_OJINAGA)
	} else if s.eq_ignore_ascii_case("America/Panama") {
		Some(AMERICA_CORAL_HARBOUR)
	} else if s.eq_ignore_ascii_case("America/Pangnirtung") {
		Some(AMERICA_PANGNIRTUNG)
	} else if s.eq_ignore_ascii_case("America/Paramaribo") {
		Some(AMERICA_PARAMARIBO)
	} else if s.eq_ignore_ascii_case("America/Phoenix") {
		Some(US_ARIZONA)
	} else if s.eq_ignore_ascii_case("America/Port-au-Prince") {
		Some(AMERICA_PORT_AU_PRINCE)
	} else if s.eq_ignore_ascii_case("America/Port_of_Spain") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Porto_Acre") {
		Some(BRAZIL_ACRE)
	} else if s.eq_ignore_ascii_case("America/Porto_Velho") {
		Some(AMERICA_PORTO_VELHO)
	} else if s.eq_ignore_ascii_case("America/Puerto_Rico") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Punta_Arenas") {
		Some(AMERICA_PUNTA_ARENAS)
	} else if s.eq_ignore_ascii_case("America/Rainy_River") {
		Some(CANADA_CENTRAL)
	} else if s.eq_ignore_ascii_case("America/Rankin_Inlet") {
		Some(AMERICA_RANKIN_INLET)
	} else if s.eq_ignore_ascii_case("America/Recife") {
		Some(AMERICA_RECIFE)
	} else if s.eq_ignore_ascii_case("America/Regina") {
		Some(CANADA_SASKATCHEWAN)
	} else if s.eq_ignore_ascii_case("America/Resolute") {
		Some(AMERICA_RESOLUTE)
	} else if s.eq_ignore_ascii_case("America/Rio_Branco") {
		Some(BRAZIL_ACRE)
	} else if s.eq_ignore_ascii_case("America/Rosario") {
		Some(AMERICA_ROSARIO)
	} else if s.eq_ignore_ascii_case("America/Santa_Isabel") {
		Some(MEXICO_BAJA_NORTE)
	} else if s.eq_ignore_ascii_case("America/Santarem") {
		Some(AMERICA_SANTAREM)
	} else if s.eq_ignore_ascii_case("America/Santiago") {
		Some(CHILE_CONTINENTAL)
	} else if s.eq_ignore_ascii_case("America/Santo_Domingo") {
		Some(AMERICA_SANTO_DOMINGO)
	} else if s.eq_ignore_ascii_case("America/Sao_Paulo") {
		Some(BRAZIL_EAST)
	} else if s.eq_ignore_ascii_case("America/Scoresbysund") {
		Some(AMERICA_SCORESBYSUND)
	} else if s.eq_ignore_ascii_case("America/Shiprock") {
		Some(US_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("America/Sitka") {
		Some(AMERICA_SITKA)
	} else if s.eq_ignore_ascii_case("America/St_Barthelemy") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/St_Johns") {
		Some(CANADA_NEWFOUNDLAND)
	} else if s.eq_ignore_ascii_case("America/St_Kitts") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/St_Lucia") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/St_Thomas") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/St_Vincent") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Swift_Current") {
		Some(AMERICA_SWIFT_CURRENT)
	} else if s.eq_ignore_ascii_case("America/Tegucigalpa") {
		Some(AMERICA_TEGUCIGALPA)
	} else if s.eq_ignore_ascii_case("America/Thule") {
		Some(AMERICA_THULE)
	} else if s.eq_ignore_ascii_case("America/Thunder_Bay") {
		Some(CANADA_EASTERN)
	} else if s.eq_ignore_ascii_case("America/Tijuana") {
		Some(MEXICO_BAJA_NORTE)
	} else if s.eq_ignore_ascii_case("America/Toronto") {
		Some(CANADA_EASTERN)
	} else if s.eq_ignore_ascii_case("America/Tortola") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Vancouver") {
		Some(CANADA_PACIFIC)
	} else if s.eq_ignore_ascii_case("America/Virgin") {
		Some(AMERICA_VIRGIN)
	} else if s.eq_ignore_ascii_case("America/Whitehorse") {
		Some(CANADA_YUKON)
	} else if s.eq_ignore_ascii_case("America/Winnipeg") {
		Some(CANADA_CENTRAL)
	} else if s.eq_ignore_ascii_case("America/Yakutat") {
		Some(AMERICA_YAKUTAT)
	} else if s.eq_ignore_ascii_case("America/Yellowknife") {
		Some(CANADA_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("Antarctica/Casey") {
		Some(ANTARCTICA_CASEY)
	} else if s.eq_ignore_ascii_case("Antarctica/Davis") {
		Some(ANTARCTICA_DAVIS)
	} else if s.eq_ignore_ascii_case("Antarctica/DumontDUrville") {
		Some(PACIFIC_YAP)
	} else if s.eq_ignore_ascii_case("Antarctica/Macquarie") {
		Some(ANTARCTICA_MACQUARIE)
	} else if s.eq_ignore_ascii_case("Antarctica/Mawson") {
		Some(ANTARCTICA_MAWSON)
	} else if s.eq_ignore_ascii_case("Antarctica/McMurdo") {
		Some(NZ)
	} else if s.eq_ignore_ascii_case("Antarctica/Palmer") {
		Some(ANTARCTICA_PALMER)
	} else if s.eq_ignore_ascii_case("Antarctica/Rothera") {
		Some(ANTARCTICA_ROTHERA)
	} else if s.eq_ignore_ascii_case("Antarctica/South_Pole") {
		Some(NZ)
	} else if s.eq_ignore_ascii_case("Antarctica/Syowa") {
		Some(ASIA_KUWAIT)
	} else if s.eq_ignore_ascii_case("Antarctica/Troll") {
		Some(ANTARCTICA_TROLL)
	} else if s.eq_ignore_ascii_case("Antarctica/Vostok") {
		Some(ASIA_KASHGAR)
	} else if s.eq_ignore_ascii_case("Arctic/Longyearbyen") {
		Some(ARCTIC_LONGYEARBYEN)
	} else if s.eq_ignore_ascii_case("Asia/Aden") {
		Some(ASIA_KUWAIT)
	} else if s.eq_ignore_ascii_case("Asia/Almaty") {
		Some(ASIA_ALMATY)
	} else if s.eq_ignore_ascii_case("Asia/Amman") {
		Some(ASIA_AMMAN)
	} else if s.eq_ignore_ascii_case("Asia/Anadyr") {
		Some(ASIA_ANADYR)
	} else if s.eq_ignore_ascii_case("Asia/Aqtau") {
		Some(ASIA_AQTAU)
	} else if s.eq_ignore_ascii_case("Asia/Aqtobe") {
		Some(ASIA_AQTOBE)
	} else if s.eq_ignore_ascii_case("Asia/Ashgabat") {
		Some(ASIA_ASHKHABAD)
	} else if s.eq_ignore_ascii_case("Asia/Ashkhabad") {
		Some(ASIA_ASHKHABAD)
	} else if s.eq_ignore_ascii_case("Asia/Atyrau") {
		Some(ASIA_ATYRAU)
	} else if s.eq_ignore_ascii_case("Asia/Baghdad") {
		Some(ASIA_BAGHDAD)
	} else if s.eq_ignore_ascii_case("Asia/Bahrain") {
		Some(ASIA_BAHRAIN)
	} else if s.eq_ignore_ascii_case("Asia/Baku") {
		Some(ASIA_BAKU)
	} else if s.eq_ignore_ascii_case("Asia/Bangkok") {
		Some(ASIA_VIENTIANE)
	} else if s.eq_ignore_ascii_case("Asia/Barnaul") {
		Some(ASIA_BARNAUL)
	} else if s.eq_ignore_ascii_case("Asia/Beirut") {
		Some(ASIA_BEIRUT)
	} else if s.eq_ignore_ascii_case("Asia/Bishkek") {
		Some(ASIA_BISHKEK)
	} else if s.eq_ignore_ascii_case("Asia/Brunei") {
		Some(ASIA_BRUNEI)
	} else if s.eq_ignore_ascii_case("Asia/Calcutta") {
		Some(ASIA_CALCUTTA)
	} else if s.eq_ignore_ascii_case("Asia/Chita") {
		Some(ASIA_CHITA)
	} else if s.eq_ignore_ascii_case("Asia/Choibalsan") {
		Some(ASIA_CHOIBALSAN)
	} else if s.eq_ignore_ascii_case("Asia/Chongqing") {
		Some(PRC)
	} else if s.eq_ignore_ascii_case("Asia/Chungking") {
		Some(PRC)
	} else if s.eq_ignore_ascii_case("Asia/Colombo") {
		Some(ASIA_COLOMBO)
	} else if s.eq_ignore_ascii_case("Asia/Dacca") {
		Some(ASIA_DACCA)
	} else if s.eq_ignore_ascii_case("Asia/Damascus") {
		Some(ASIA_DAMASCUS)
	} else if s.eq_ignore_ascii_case("Asia/Dhaka") {
		Some(ASIA_DACCA)
	} else if s.eq_ignore_ascii_case("Asia/Dili") {
		Some(ASIA_DILI)
	} else if s.eq_ignore_ascii_case("Asia/Dubai") {
		Some(ASIA_MUSCAT)
	} else if s.eq_ignore_ascii_case("Asia/Dushanbe") {
		Some(ASIA_DUSHANBE)
	} else if s.eq_ignore_ascii_case("Asia/Famagusta") {
		Some(ASIA_FAMAGUSTA)
	} else if s.eq_ignore_ascii_case("Asia/Gaza") {
		Some(ASIA_GAZA)
	} else if s.eq_ignore_ascii_case("Asia/Harbin") {
		Some(PRC)
	} else if s.eq_ignore_ascii_case("Asia/Hebron") {
		Some(ASIA_HEBRON)
	} else if s.eq_ignore_ascii_case("Asia/Ho_Chi_Minh") {
		Some(ASIA_SAIGON)
	} else if s.eq_ignore_ascii_case("Asia/Hong_Kong") {
		Some(HONGKONG)
	} else if s.eq_ignore_ascii_case("Asia/Hovd") {
		Some(ASIA_HOVD)
	} else if s.eq_ignore_ascii_case("Asia/Irkutsk") {
		Some(ASIA_IRKUTSK)
	} else if s.eq_ignore_ascii_case("Asia/Istanbul") {
		Some(TURKEY)
	} else if s.eq_ignore_ascii_case("Asia/Jakarta") {
		Some(ASIA_JAKARTA)
	} else if s.eq_ignore_ascii_case("Asia/Jayapura") {
		Some(ASIA_JAYAPURA)
	} else if s.eq_ignore_ascii_case("Asia/Jerusalem") {
		Some(ISRAEL)
	} else if s.eq_ignore_ascii_case("Asia/Kabul") {
		Some(ASIA_KABUL)
	} else if s.eq_ignore_ascii_case("Asia/Kamchatka") {
		Some(ASIA_KAMCHATKA)
	} else if s.eq_ignore_ascii_case("Asia/Karachi") {
		Some(ASIA_KARACHI)
	} else if s.eq_ignore_ascii_case("Asia/Kashgar") {
		Some(ASIA_KASHGAR)
	} else if s.eq_ignore_ascii_case("Asia/Kathmandu") {
		Some(ASIA_KATMANDU)
	} else if s.eq_ignore_ascii_case("Asia/Katmandu") {
		Some(ASIA_KATMANDU)
	} else if s.eq_ignore_ascii_case("Asia/Khandyga") {
		Some(ASIA_KHANDYGA)
	} else if s.eq_ignore_ascii_case("Asia/Kolkata") {
		Some(ASIA_CALCUTTA)
	} else if s.eq_ignore_ascii_case("Asia/Krasnoyarsk") {
		Some(ASIA_KRASNOYARSK)
	} else if s.eq_ignore_ascii_case("Asia/Kuala_Lumpur") {
		Some(SINGAPORE)
	} else if s.eq_ignore_ascii_case("Asia/Kuching") {
		Some(ASIA_BRUNEI)
	} else if s.eq_ignore_ascii_case("Asia/Kuwait") {
		Some(ASIA_KUWAIT)
	} else if s.eq_ignore_ascii_case("Asia/Macao") {
		Some(ASIA_MACAO)
	} else if s.eq_ignore_ascii_case("Asia/Macau") {
		Some(ASIA_MACAO)
	} else if s.eq_ignore_ascii_case("Asia/Magadan") {
		Some(ASIA_MAGADAN)
	} else if s.eq_ignore_ascii_case("Asia/Makassar") {
		Some(ASIA_UJUNG_PANDANG)
	} else if s.eq_ignore_ascii_case("Asia/Manila") {
		Some(ASIA_MANILA)
	} else if s.eq_ignore_ascii_case("Asia/Muscat") {
		Some(ASIA_MUSCAT)
	} else if s.eq_ignore_ascii_case("Asia/Nicosia") {
		Some(EUROPE_NICOSIA)
	} else if s.eq_ignore_ascii_case("Asia/Novokuznetsk") {
		Some(ASIA_NOVOKUZNETSK)
	} else if s.eq_ignore_ascii_case("Asia/Novosibirsk") {
		Some(ASIA_NOVOSIBIRSK)
	} else if s.eq_ignore_ascii_case("Asia/Omsk") {
		Some(ASIA_OMSK)
	} else if s.eq_ignore_ascii_case("Asia/Oral") {
		Some(ASIA_ORAL)
	} else if s.eq_ignore_ascii_case("Asia/Phnom_Penh") {
		Some(ASIA_VIENTIANE)
	} else if s.eq_ignore_ascii_case("Asia/Pontianak") {
		Some(ASIA_PONTIANAK)
	} else if s.eq_ignore_ascii_case("Asia/Pyongyang") {
		Some(ASIA_PYONGYANG)
	} else if s.eq_ignore_ascii_case("Asia/Qatar") {
		Some(ASIA_BAHRAIN)
	} else if s.eq_ignore_ascii_case("Asia/Qostanay") {
		Some(ASIA_QOSTANAY)
	} else if s.eq_ignore_ascii_case("Asia/Qyzylorda") {
		Some(ASIA_QYZYLORDA)
	} else if s.eq_ignore_ascii_case("Asia/Rangoon") {
		Some(ASIA_RANGOON)
	} else if s.eq_ignore_ascii_case("Asia/Riyadh") {
		Some(ASIA_KUWAIT)
	} else if s.eq_ignore_ascii_case("Asia/Saigon") {
		Some(ASIA_SAIGON)
	} else if s.eq_ignore_ascii_case("Asia/Sakhalin") {
		Some(ASIA_SAKHALIN)
	} else if s.eq_ignore_ascii_case("Asia/Samarkand") {
		Some(ASIA_SAMARKAND)
	} else if s.eq_ignore_ascii_case("Asia/Seoul") {
		Some(ROK)
	} else if s.eq_ignore_ascii_case("Asia/Shanghai") {
		Some(PRC)
	} else if s.eq_ignore_ascii_case("Asia/Singapore") {
		Some(SINGAPORE)
	} else if s.eq_ignore_ascii_case("Asia/Srednekolymsk") {
		Some(ASIA_SREDNEKOLYMSK)
	} else if s.eq_ignore_ascii_case("Asia/Taipei") {
		Some(ROC)
	} else if s.eq_ignore_ascii_case("Asia/Tashkent") {
		Some(ASIA_TASHKENT)
	} else if s.eq_ignore_ascii_case("Asia/Tbilisi") {
		Some(ASIA_TBILISI)
	} else if s.eq_ignore_ascii_case("Asia/Tehran") {
		Some(IRAN)
	} else if s.eq_ignore_ascii_case("Asia/Tel_Aviv") {
		Some(ISRAEL)
	} else if s.eq_ignore_ascii_case("Asia/Thimbu") {
		Some(ASIA_THIMBU)
	} else if s.eq_ignore_ascii_case("Asia/Thimphu") {
		Some(ASIA_THIMBU)
	} else if s.eq_ignore_ascii_case("Asia/Tokyo") {
		Some(JAPAN)
	} else if s.eq_ignore_ascii_case("Asia/Tomsk") {
		Some(ASIA_TOMSK)
	} else if s.eq_ignore_ascii_case("Asia/Ujung_Pandang") {
		Some(ASIA_UJUNG_PANDANG)
	} else if s.eq_ignore_ascii_case("Asia/Ulaanbaatar") {
		Some(ASIA_ULAN_BATOR)
	} else if s.eq_ignore_ascii_case("Asia/Ulan_Bator") {
		Some(ASIA_ULAN_BATOR)
	} else if s.eq_ignore_ascii_case("Asia/Urumqi") {
		Some(ASIA_KASHGAR)
	} else if s.eq_ignore_ascii_case("Asia/Ust-Nera") {
		Some(ASIA_UST_NERA)
	} else if s.eq_ignore_ascii_case("Asia/Vientiane") {
		Some(ASIA_VIENTIANE)
	} else if s.eq_ignore_ascii_case("Asia/Vladivostok") {
		Some(ASIA_VLADIVOSTOK)
	} else if s.eq_ignore_ascii_case("Asia/Yakutsk") {
		Some(ASIA_YAKUTSK)
	} else if s.eq_ignore_ascii_case("Asia/Yangon") {
		Some(ASIA_RANGOON)
	} else if s.eq_ignore_ascii_case("Asia/Yekaterinburg") {
		Some(ASIA_YEKATERINBURG)
	} else if s.eq_ignore_ascii_case("Asia/Yerevan") {
		Some(ASIA_YEREVAN)
	} else if s.eq_ignore_ascii_case("Atlantic/Azores") {
		Some(ATLANTIC_AZORES)
	} else if s.eq_ignore_ascii_case("Atlantic/Bermuda") {
		Some(ATLANTIC_BERMUDA)
	} else if s.eq_ignore_ascii_case("Atlantic/Canary") {
		Some(ATLANTIC_CANARY)
	} else if s.eq_ignore_ascii_case("Atlantic/Cape_Verde") {
		Some(ATLANTIC_CAPE_VERDE)
	} else if s.eq_ignore_ascii_case("Atlantic/Faeroe") {
		Some(ATLANTIC_FAEROE)
	} else if s.eq_ignore_ascii_case("Atlantic/Faroe") {
		Some(ATLANTIC_FAEROE)
	} else if s.eq_ignore_ascii_case("Atlantic/Jan_Mayen") {
		Some(ARCTIC_LONGYEARBYEN)
	} else if s.eq_ignore_ascii_case("Atlantic/Madeira") {
		Some(ATLANTIC_MADEIRA)
	} else if s.eq_ignore_ascii_case("Atlantic/Reykjavik") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Atlantic/South_Georgia") {
		Some(ATLANTIC_SOUTH_GEORGIA)
	} else if s.eq_ignore_ascii_case("Atlantic/St_Helena") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Atlantic/Stanley") {
		Some(ATLANTIC_STANLEY)
	} else if s.eq_ignore_ascii_case("Australia/ACT") {
		Some(AUSTRALIA_NSW)
	} else if s.eq_ignore_ascii_case("Australia/Adelaide") {
		Some(AUSTRALIA_SOUTH)
	} else if s.eq_ignore_ascii_case("Australia/Brisbane") {
		Some(AUSTRALIA_QUEENSLAND)
	} else if s.eq_ignore_ascii_case("Australia/Broken_Hill") {
		Some(AUSTRALIA_YANCOWINNA)
	} else if s.eq_ignore_ascii_case("Australia/Canberra") {
		Some(AUSTRALIA_NSW)
	} else if s.eq_ignore_ascii_case("Australia/Currie") {
		Some(AUSTRALIA_TASMANIA)
	} else if s.eq_ignore_ascii_case("Australia/Darwin") {
		Some(AUSTRALIA_NORTH)
	} else if s.eq_ignore_ascii_case("Australia/Eucla") {
		Some(AUSTRALIA_EUCLA)
	} else if s.eq_ignore_ascii_case("Australia/Hobart") {
		Some(AUSTRALIA_TASMANIA)
	} else if s.eq_ignore_ascii_case("Australia/LHI") {
		Some(AUSTRALIA_LHI)
	} else if s.eq_ignore_ascii_case("Australia/Lindeman") {
		Some(AUSTRALIA_LINDEMAN)
	} else if s.eq_ignore_ascii_case("Australia/Lord_Howe") {
		Some(AUSTRALIA_LHI)
	} else if s.eq_ignore_ascii_case("Australia/Melbourne") {
		Some(AUSTRALIA_VICTORIA)
	} else if s.eq_ignore_ascii_case("Australia/NSW") {
		Some(AUSTRALIA_NSW)
	} else if s.eq_ignore_ascii_case("Australia/North") {
		Some(AUSTRALIA_NORTH)
	} else if s.eq_ignore_ascii_case("Australia/Perth") {
		Some(AUSTRALIA_WEST)
	} else if s.eq_ignore_ascii_case("Australia/Queensland") {
		Some(AUSTRALIA_QUEENSLAND)
	} else if s.eq_ignore_ascii_case("Australia/South") {
		Some(AUSTRALIA_SOUTH)
	} else if s.eq_ignore_ascii_case("Australia/Sydney") {
		Some(AUSTRALIA_NSW)
	} else if s.eq_ignore_ascii_case("Australia/Tasmania") {
		Some(AUSTRALIA_TASMANIA)
	} else if s.eq_ignore_ascii_case("Australia/Victoria") {
		Some(AUSTRALIA_VICTORIA)
	} else if s.eq_ignore_ascii_case("Australia/West") {
		Some(AUSTRALIA_WEST)
	} else if s.eq_ignore_ascii_case("Australia/Yancowinna") {
		Some(AUSTRALIA_YANCOWINNA)
	} else if s.eq_ignore_ascii_case("Brazil/Acre") {
		Some(BRAZIL_ACRE)
	} else if s.eq_ignore_ascii_case("Brazil/DeNoronha") {
		Some(BRAZIL_DE_NORONHA)
	} else if s.eq_ignore_ascii_case("Brazil/East") {
		Some(BRAZIL_EAST)
	} else if s.eq_ignore_ascii_case("Brazil/West") {
		Some(BRAZIL_WEST)
	} else if s.eq_ignore_ascii_case("CET") {
		Some(CET)
	} else if s.eq_ignore_ascii_case("CST6CDT") {
		Some(CST_6_CDT)
	} else if s.eq_ignore_ascii_case("Canada/Atlantic") {
		Some(CANADA_ATLANTIC)
	} else if s.eq_ignore_ascii_case("Canada/Central") {
		Some(CANADA_CENTRAL)
	} else if s.eq_ignore_ascii_case("Canada/Eastern") {
		Some(CANADA_EASTERN)
	} else if s.eq_ignore_ascii_case("Canada/Mountain") {
		Some(CANADA_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("Canada/Newfoundland") {
		Some(CANADA_NEWFOUNDLAND)
	} else if s.eq_ignore_ascii_case("Canada/Pacific") {
		Some(CANADA_PACIFIC)
	} else if s.eq_ignore_ascii_case("Canada/Saskatchewan") {
		Some(CANADA_SASKATCHEWAN)
	} else if s.eq_ignore_ascii_case("Canada/Yukon") {
		Some(CANADA_YUKON)
	} else if s.eq_ignore_ascii_case("Chile/Continental") {
		Some(CHILE_CONTINENTAL)
	} else if s.eq_ignore_ascii_case("Chile/EasterIsland") {
		Some(CHILE_EASTER_ISLAND)
	} else if s.eq_ignore_ascii_case("Cuba") {
		Some(CUBA)
	} else if s.eq_ignore_ascii_case("EET") {
		Some(EET)
	} else if s.eq_ignore_ascii_case("EST") {
		Some(EST)
	} else if s.eq_ignore_ascii_case("EST5EDT") {
		Some(EST_5_EDT)
	} else if s.eq_ignore_ascii_case("Egypt") {
		Some(EGYPT)
	} else if s.eq_ignore_ascii_case("Eire") {
		Some(EIRE)
	} else if s.eq_ignore_ascii_case("Etc/GMT") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("Etc/GMT+0") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("Etc/GMT+1") {
		Some(ETC_GMT_PLUS_1)
	} else if s.eq_ignore_ascii_case("Etc/GMT+10") {
		Some(ETC_GMT_PLUS_10)
	} else if s.eq_ignore_ascii_case("Etc/GMT+11") {
		Some(ETC_GMT_PLUS_11)
	} else if s.eq_ignore_ascii_case("Etc/GMT+12") {
		Some(ETC_GMT_PLUS_12)
	} else if s.eq_ignore_ascii_case("Etc/GMT+2") {
		Some(ETC_GMT_PLUS_2)
	} else if s.eq_ignore_ascii_case("Etc/GMT+3") {
		Some(ETC_GMT_PLUS_3)
	} else if s.eq_ignore_ascii_case("Etc/GMT+4") {
		Some(ETC_GMT_PLUS_4)
	} else if s.eq_ignore_ascii_case("Etc/GMT+5") {
		Some(ETC_GMT_PLUS_5)
	} else if s.eq_ignore_ascii_case("Etc/GMT+6") {
		Some(ETC_GMT_PLUS_6)
	} else if s.eq_ignore_ascii_case("Etc/GMT+7") {
		Some(ETC_GMT_PLUS_7)
	} else if s.eq_ignore_ascii_case("Etc/GMT+8") {
		Some(ETC_GMT_PLUS_8)
	} else if s.eq_ignore_ascii_case("Etc/GMT+9") {
		Some(ETC_GMT_PLUS_9)
	} else if s.eq_ignore_ascii_case("Etc/GMT-0") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("Etc/GMT-1") {
		Some(ETC_GMT_MINUS_1)
	} else if s.eq_ignore_ascii_case("Etc/GMT-10") {
		Some(ETC_GMT_MINUS_10)
	} else if s.eq_ignore_ascii_case("Etc/GMT-11") {
		Some(ETC_GMT_MINUS_11)
	} else if s.eq_ignore_ascii_case("Etc/GMT-12") {
		Some(ETC_GMT_MINUS_12)
	} else if s.eq_ignore_ascii_case("Etc/GMT-13") {
		Some(ETC_GMT_MINUS_13)
	} else if s.eq_ignore_ascii_case("Etc/GMT-14") {
		Some(ETC_GMT_MINUS_14)
	} else if s.eq_ignore_ascii_case("Etc/GMT-2") {
		Some(ETC_GMT_MINUS_2)
	} else if s.eq_ignore_ascii_case("Etc/GMT-3") {
		Some(ETC_GMT_MINUS_3)
	} else if s.eq_ignore_ascii_case("Etc/GMT-4") {
		Some(ETC_GMT_MINUS_4)
	} else if s.eq_ignore_ascii_case("Etc/GMT-5") {
		Some(ETC_GMT_MINUS_5)
	} else if s.eq_ignore_ascii_case("Etc/GMT-6") {
		Some(ETC_GMT_MINUS_6)
	} else if s.eq_ignore_ascii_case("Etc/GMT-7") {
		Some(ETC_GMT_MINUS_7)
	} else if s.eq_ignore_ascii_case("Etc/GMT-8") {
		Some(ETC_GMT_MINUS_8)
	} else if s.eq_ignore_ascii_case("Etc/GMT-9") {
		Some(ETC_GMT_MINUS_9)
	} else if s.eq_ignore_ascii_case("Etc/GMT0") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("Etc/Greenwich") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("Etc/UCT") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("Etc/UTC") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("Etc/Universal") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("Etc/Zulu") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("Europe/Amsterdam") {
		Some(EUROPE_LUXEMBOURG)
	} else if s.eq_ignore_ascii_case("Europe/Andorra") {
		Some(EUROPE_ANDORRA)
	} else if s.eq_ignore_ascii_case("Europe/Astrakhan") {
		Some(EUROPE_ASTRAKHAN)
	} else if s.eq_ignore_ascii_case("Europe/Athens") {
		Some(EUROPE_ATHENS)
	} else if s.eq_ignore_ascii_case("Europe/Belfast") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("Europe/Belgrade") {
		Some(EUROPE_ZAGREB)
	} else if s.eq_ignore_ascii_case("Europe/Berlin") {
		Some(ARCTIC_LONGYEARBYEN)
	} else if s.eq_ignore_ascii_case("Europe/Bratislava") {
		Some(EUROPE_BRATISLAVA)
	} else if s.eq_ignore_ascii_case("Europe/Brussels") {
		Some(EUROPE_LUXEMBOURG)
	} else if s.eq_ignore_ascii_case("Europe/Bucharest") {
		Some(EUROPE_BUCHAREST)
	} else if s.eq_ignore_ascii_case("Europe/Budapest") {
		Some(EUROPE_BUDAPEST)
	} else if s.eq_ignore_ascii_case("Europe/Busingen") {
		Some(EUROPE_VADUZ)
	} else if s.eq_ignore_ascii_case("Europe/Chisinau") {
		Some(EUROPE_TIRASPOL)
	} else if s.eq_ignore_ascii_case("Europe/Copenhagen") {
		Some(ARCTIC_LONGYEARBYEN)
	} else if s.eq_ignore_ascii_case("Europe/Dublin") {
		Some(EIRE)
	} else if s.eq_ignore_ascii_case("Europe/Gibraltar") {
		Some(EUROPE_GIBRALTAR)
	} else if s.eq_ignore_ascii_case("Europe/Guernsey") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("Europe/Helsinki") {
		Some(EUROPE_MARIEHAMN)
	} else if s.eq_ignore_ascii_case("Europe/Isle_of_Man") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("Europe/Istanbul") {
		Some(TURKEY)
	} else if s.eq_ignore_ascii_case("Europe/Jersey") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("Europe/Kaliningrad") {
		Some(EUROPE_KALININGRAD)
	} else if s.eq_ignore_ascii_case("Europe/Kiev") {
		Some(EUROPE_ZAPOROZHYE)
	} else if s.eq_ignore_ascii_case("Europe/Kirov") {
		Some(EUROPE_KIROV)
	} else if s.eq_ignore_ascii_case("Europe/Kyiv") {
		Some(EUROPE_ZAPOROZHYE)
	} else if s.eq_ignore_ascii_case("Europe/Lisbon") {
		Some(PORTUGAL)
	} else if s.eq_ignore_ascii_case("Europe/Ljubljana") {
		Some(EUROPE_ZAGREB)
	} else if s.eq_ignore_ascii_case("Europe/London") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("Europe/Luxembourg") {
		Some(EUROPE_LUXEMBOURG)
	} else if s.eq_ignore_ascii_case("Europe/Madrid") {
		Some(EUROPE_MADRID)
	} else if s.eq_ignore_ascii_case("Europe/Malta") {
		Some(EUROPE_MALTA)
	} else if s.eq_ignore_ascii_case("Europe/Mariehamn") {
		Some(EUROPE_MARIEHAMN)
	} else if s.eq_ignore_ascii_case("Europe/Minsk") {
		Some(EUROPE_MINSK)
	} else if s.eq_ignore_ascii_case("Europe/Monaco") {
		Some(EUROPE_MONACO)
	} else if s.eq_ignore_ascii_case("Europe/Moscow") {
		Some(W_SU)
	} else if s.eq_ignore_ascii_case("Europe/Nicosia") {
		Some(EUROPE_NICOSIA)
	} else if s.eq_ignore_ascii_case("Europe/Oslo") {
		Some(ARCTIC_LONGYEARBYEN)
	} else if s.eq_ignore_ascii_case("Europe/Paris") {
		Some(EUROPE_MONACO)
	} else if s.eq_ignore_ascii_case("Europe/Podgorica") {
		Some(EUROPE_ZAGREB)
	} else if s.eq_ignore_ascii_case("Europe/Prague") {
		Some(EUROPE_BRATISLAVA)
	} else if s.eq_ignore_ascii_case("Europe/Riga") {
		Some(EUROPE_RIGA)
	} else if s.eq_ignore_ascii_case("Europe/Rome") {
		Some(EUROPE_VATICAN)
	} else if s.eq_ignore_ascii_case("Europe/Samara") {
		Some(EUROPE_SAMARA)
	} else if s.eq_ignore_ascii_case("Europe/San_Marino") {
		Some(EUROPE_VATICAN)
	} else if s.eq_ignore_ascii_case("Europe/Sarajevo") {
		Some(EUROPE_ZAGREB)
	} else if s.eq_ignore_ascii_case("Europe/Saratov") {
		Some(EUROPE_SARATOV)
	} else if s.eq_ignore_ascii_case("Europe/Simferopol") {
		Some(EUROPE_SIMFEROPOL)
	} else if s.eq_ignore_ascii_case("Europe/Skopje") {
		Some(EUROPE_ZAGREB)
	} else if s.eq_ignore_ascii_case("Europe/Sofia") {
		Some(EUROPE_SOFIA)
	} else if s.eq_ignore_ascii_case("Europe/Stockholm") {
		Some(ARCTIC_LONGYEARBYEN)
	} else if s.eq_ignore_ascii_case("Europe/Tallinn") {
		Some(EUROPE_TALLINN)
	} else if s.eq_ignore_ascii_case("Europe/Tirane") {
		Some(EUROPE_TIRANE)
	} else if s.eq_ignore_ascii_case("Europe/Tiraspol") {
		Some(EUROPE_TIRASPOL)
	} else if s.eq_ignore_ascii_case("Europe/Ulyanovsk") {
		Some(EUROPE_ULYANOVSK)
	} else if s.eq_ignore_ascii_case("Europe/Uzhgorod") {
		Some(EUROPE_ZAPOROZHYE)
	} else if s.eq_ignore_ascii_case("Europe/Vaduz") {
		Some(EUROPE_VADUZ)
	} else if s.eq_ignore_ascii_case("Europe/Vatican") {
		Some(EUROPE_VATICAN)
	} else if s.eq_ignore_ascii_case("Europe/Vienna") {
		Some(EUROPE_VIENNA)
	} else if s.eq_ignore_ascii_case("Europe/Vilnius") {
		Some(EUROPE_VILNIUS)
	} else if s.eq_ignore_ascii_case("Europe/Volgograd") {
		Some(EUROPE_VOLGOGRAD)
	} else if s.eq_ignore_ascii_case("Europe/Warsaw") {
		Some(POLAND)
	} else if s.eq_ignore_ascii_case("Europe/Zagreb") {
		Some(EUROPE_ZAGREB)
	} else if s.eq_ignore_ascii_case("Europe/Zaporozhye") {
		Some(EUROPE_ZAPOROZHYE)
	} else if s.eq_ignore_ascii_case("Europe/Zurich") {
		Some(EUROPE_VADUZ)
	} else if s.eq_ignore_ascii_case("Factory") {
		Some(FACTORY)
	} else if s.eq_ignore_ascii_case("GB") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("GB-Eire") {
		Some(GB_EIRE)
	} else if s.eq_ignore_ascii_case("GMT") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("GMT+0") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("GMT-0") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("GMT0") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("Greenwich") {
		Some(GREENWICH)
	} else if s.eq_ignore_ascii_case("HST") {
		Some(HST)
	} else if s.eq_ignore_ascii_case("Hongkong") {
		Some(HONGKONG)
	} else if s.eq_ignore_ascii_case("Iceland") {
		Some(ICELAND)
	} else if s.eq_ignore_ascii_case("Indian/Antananarivo") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Indian/Chagos") {
		Some(INDIAN_CHAGOS)
	} else if s.eq_ignore_ascii_case("Indian/Christmas") {
		Some(ASIA_VIENTIANE)
	} else if s.eq_ignore_ascii_case("Indian/Cocos") {
		Some(ASIA_RANGOON)
	} else if s.eq_ignore_ascii_case("Indian/Comoro") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Indian/Kerguelen") {
		Some(INDIAN_KERGUELEN)
	} else if s.eq_ignore_ascii_case("Indian/Mahe") {
		Some(ASIA_MUSCAT)
	} else if s.eq_ignore_ascii_case("Indian/Maldives") {
		Some(INDIAN_KERGUELEN)
	} else if s.eq_ignore_ascii_case("Indian/Mauritius") {
		Some(INDIAN_MAURITIUS)
	} else if s.eq_ignore_ascii_case("Indian/Mayotte") {
		Some(INDIAN_MAYOTTE)
	} else if s.eq_ignore_ascii_case("Indian/Reunion") {
		Some(ASIA_MUSCAT)
	} else if s.eq_ignore_ascii_case("Iran") {
		Some(IRAN)
	} else if s.eq_ignore_ascii_case("Israel") {
		Some(ISRAEL)
	} else if s.eq_ignore_ascii_case("Jamaica") {
		Some(JAMAICA)
	} else if s.eq_ignore_ascii_case("Japan") {
		Some(JAPAN)
	} else if s.eq_ignore_ascii_case("Kwajalein") {
		Some(KWAJALEIN)
	} else if s.eq_ignore_ascii_case("Libya") {
		Some(LIBYA)
	} else if s.eq_ignore_ascii_case("MET") {
		Some(MET)
	} else if s.eq_ignore_ascii_case("MST") {
		Some(MST)
	} else if s.eq_ignore_ascii_case("MST7MDT") {
		Some(MST_7_MDT)
	} else if s.eq_ignore_ascii_case("Mexico/BajaNorte") {
		Some(MEXICO_BAJA_NORTE)
	} else if s.eq_ignore_ascii_case("Mexico/BajaSur") {
		Some(MEXICO_BAJA_SUR)
	} else if s.eq_ignore_ascii_case("Mexico/General") {
		Some(MEXICO_GENERAL)
	} else if s.eq_ignore_ascii_case("NZ") {
		Some(NZ)
	} else if s.eq_ignore_ascii_case("NZ-CHAT") {
		Some(NZ_CHAT)
	} else if s.eq_ignore_ascii_case("Navajo") {
		Some(US_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("PRC") {
		Some(PRC)
	} else if s.eq_ignore_ascii_case("PST8PDT") {
		Some(PST_8_PDT)
	} else if s.eq_ignore_ascii_case("Pacific/Apia") {
		Some(PACIFIC_APIA)
	} else if s.eq_ignore_ascii_case("Pacific/Auckland") {
		Some(NZ)
	} else if s.eq_ignore_ascii_case("Pacific/Bougainville") {
		Some(PACIFIC_BOUGAINVILLE)
	} else if s.eq_ignore_ascii_case("Pacific/Chatham") {
		Some(NZ_CHAT)
	} else if s.eq_ignore_ascii_case("Pacific/Chuuk") {
		Some(PACIFIC_YAP)
	} else if s.eq_ignore_ascii_case("Pacific/Easter") {
		Some(CHILE_EASTER_ISLAND)
	} else if s.eq_ignore_ascii_case("Pacific/Efate") {
		Some(PACIFIC_EFATE)
	} else if s.eq_ignore_ascii_case("Pacific/Enderbury") {
		Some(PACIFIC_ENDERBURY)
	} else if s.eq_ignore_ascii_case("Pacific/Fakaofo") {
		Some(PACIFIC_FAKAOFO)
	} else if s.eq_ignore_ascii_case("Pacific/Fiji") {
		Some(PACIFIC_FIJI)
	} else if s.eq_ignore_ascii_case("Pacific/Funafuti") {
		Some(PACIFIC_WALLIS)
	} else if s.eq_ignore_ascii_case("Pacific/Galapagos") {
		Some(PACIFIC_GALAPAGOS)
	} else if s.eq_ignore_ascii_case("Pacific/Gambier") {
		Some(PACIFIC_GAMBIER)
	} else if s.eq_ignore_ascii_case("Pacific/Guadalcanal") {
		Some(PACIFIC_PONAPE)
	} else if s.eq_ignore_ascii_case("Pacific/Guam") {
		Some(PACIFIC_SAIPAN)
	} else if s.eq_ignore_ascii_case("Pacific/Honolulu") {
		Some(US_HAWAII)
	} else if s.eq_ignore_ascii_case("Pacific/Johnston") {
		Some(US_HAWAII)
	} else if s.eq_ignore_ascii_case("Pacific/Kanton") {
		Some(PACIFIC_ENDERBURY)
	} else if s.eq_ignore_ascii_case("Pacific/Kiritimati") {
		Some(PACIFIC_KIRITIMATI)
	} else if s.eq_ignore_ascii_case("Pacific/Kosrae") {
		Some(PACIFIC_KOSRAE)
	} else if s.eq_ignore_ascii_case("Pacific/Kwajalein") {
		Some(KWAJALEIN)
	} else if s.eq_ignore_ascii_case("Pacific/Majuro") {
		Some(PACIFIC_WALLIS)
	} else if s.eq_ignore_ascii_case("Pacific/Marquesas") {
		Some(PACIFIC_MARQUESAS)
	} else if s.eq_ignore_ascii_case("Pacific/Midway") {
		Some(US_SAMOA)
	} else if s.eq_ignore_ascii_case("Pacific/Nauru") {
		Some(PACIFIC_NAURU)
	} else if s.eq_ignore_ascii_case("Pacific/Niue") {
		Some(PACIFIC_NIUE)
	} else if s.eq_ignore_ascii_case("Pacific/Norfolk") {
		Some(PACIFIC_NORFOLK)
	} else if s.eq_ignore_ascii_case("Pacific/Noumea") {
		Some(PACIFIC_NOUMEA)
	} else if s.eq_ignore_ascii_case("Pacific/Pago_Pago") {
		Some(US_SAMOA)
	} else if s.eq_ignore_ascii_case("Pacific/Palau") {
		Some(PACIFIC_PALAU)
	} else if s.eq_ignore_ascii_case("Pacific/Pitcairn") {
		Some(PACIFIC_PITCAIRN)
	} else if s.eq_ignore_ascii_case("Pacific/Pohnpei") {
		Some(PACIFIC_PONAPE)
	} else if s.eq_ignore_ascii_case("Pacific/Ponape") {
		Some(PACIFIC_PONAPE)
	} else if s.eq_ignore_ascii_case("Pacific/Port_Moresby") {
		Some(PACIFIC_YAP)
	} else if s.eq_ignore_ascii_case("Pacific/Rarotonga") {
		Some(PACIFIC_RAROTONGA)
	} else if s.eq_ignore_ascii_case("Pacific/Saipan") {
		Some(PACIFIC_SAIPAN)
	} else if s.eq_ignore_ascii_case("Pacific/Samoa") {
		Some(US_SAMOA)
	} else if s.eq_ignore_ascii_case("Pacific/Tahiti") {
		Some(PACIFIC_TAHITI)
	} else if s.eq_ignore_ascii_case("Pacific/Tarawa") {
		Some(PACIFIC_WALLIS)
	} else if s.eq_ignore_ascii_case("Pacific/Tongatapu") {
		Some(PACIFIC_TONGATAPU)
	} else if s.eq_ignore_ascii_case("Pacific/Truk") {
		Some(PACIFIC_YAP)
	} else if s.eq_ignore_ascii_case("Pacific/Wake") {
		Some(PACIFIC_WALLIS)
	} else if s.eq_ignore_ascii_case("Pacific/Wallis") {
		Some(PACIFIC_WALLIS)
	} else if s.eq_ignore_ascii_case("Pacific/Yap") {
		Some(PACIFIC_YAP)
	} else if s.eq_ignore_ascii_case("Poland") {
		Some(POLAND)
	} else if s.eq_ignore_ascii_case("Portugal") {
		Some(PORTUGAL)
	} else if s.eq_ignore_ascii_case("ROC") {
		Some(ROC)
	} else if s.eq_ignore_ascii_case("ROK") {
		Some(ROK)
	} else if s.eq_ignore_ascii_case("Singapore") {
		Some(SINGAPORE)
	} else if s.eq_ignore_ascii_case("Turkey") {
		Some(TURKEY)
	} else if s.eq_ignore_ascii_case("UCT") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("US/Alaska") {
		Some(US_ALASKA)
	} else if s.eq_ignore_ascii_case("US/Aleutian") {
		Some(US_ALEUTIAN)
	} else if s.eq_ignore_ascii_case("US/Arizona") {
		Some(US_ARIZONA)
	} else if s.eq_ignore_ascii_case("US/Central") {
		Some(US_CENTRAL)
	} else if s.eq_ignore_ascii_case("US/East-Indiana") {
		Some(US_EAST_INDIANA)
	} else if s.eq_ignore_ascii_case("US/Eastern") {
		Some(US_EASTERN)
	} else if s.eq_ignore_ascii_case("US/Hawaii") {
		Some(US_HAWAII)
	} else if s.eq_ignore_ascii_case("US/Indiana-Starke") {
		Some(US_INDIANA_STARKE)
	} else if s.eq_ignore_ascii_case("US/Michigan") {
		Some(US_MICHIGAN)
	} else if s.eq_ignore_ascii_case("US/Mountain") {
		Some(US_MOUNTAIN)
	} else if s.eq_ignore_ascii_case("US/Pacific") {
		Some(US_PACIFIC)
	} else if s.eq_ignore_ascii_case("US/Samoa") {
		Some(US_SAMOA)
	} else if s.eq_ignore_ascii_case("UTC") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("Universal") {
		Some(ZULU)
	} else if s.eq_ignore_ascii_case("W-SU") {
		Some(W_SU)
	} else if s.eq_ignore_ascii_case("WET") {
		Some(WET)
	} else if s.eq_ignore_ascii_case("Zulu") {
		Some(ZULU)
	} else {
		None
	}
}
pub mod time_zones {
	use super::*;
	pub mod Africa {
		use super::*;
		/// The `Africa/Abidjan` time zone.
		pub struct Abidjan;
		impl Sealed for Abidjan {
			const NAME: &'static str = "Africa/Abidjan";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Abidjan {}
		/// The `Africa/Accra` time zone.
		pub struct Accra;
		impl Sealed for Accra {
			const NAME: &'static str = "Africa/Accra";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Accra {}
		/// The `Africa/Addis_Ababa` time zone.
		pub struct AddisAbaba;
		impl Sealed for AddisAbaba {
			const NAME: &'static str = "Africa/Addis_Ababa";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for AddisAbaba {}
		/// The `Africa/Algiers` time zone.
		pub struct Algiers;
		impl Sealed for Algiers {
			const NAME: &'static str = "Africa/Algiers";
			const DATA: TzifData = AFRICA_ALGIERS;
		}
		impl TimeZone for Algiers {}
		/// The `Africa/Asmara` time zone.
		pub struct Asmara;
		impl Sealed for Asmara {
			const NAME: &'static str = "Africa/Asmara";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Asmara {}
		/// The `Africa/Asmera` time zone.
		pub struct Asmera;
		impl Sealed for Asmera {
			const NAME: &'static str = "Africa/Asmera";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Asmera {}
		/// The `Africa/Bamako` time zone.
		pub struct Bamako;
		impl Sealed for Bamako {
			const NAME: &'static str = "Africa/Bamako";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Bamako {}
		/// The `Africa/Bangui` time zone.
		pub struct Bangui;
		impl Sealed for Bangui {
			const NAME: &'static str = "Africa/Bangui";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Bangui {}
		/// The `Africa/Banjul` time zone.
		pub struct Banjul;
		impl Sealed for Banjul {
			const NAME: &'static str = "Africa/Banjul";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Banjul {}
		/// The `Africa/Bissau` time zone.
		pub struct Bissau;
		impl Sealed for Bissau {
			const NAME: &'static str = "Africa/Bissau";
			const DATA: TzifData = AFRICA_BISSAU;
		}
		impl TimeZone for Bissau {}
		/// The `Africa/Blantyre` time zone.
		pub struct Blantyre;
		impl Sealed for Blantyre {
			const NAME: &'static str = "Africa/Blantyre";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Blantyre {}
		/// The `Africa/Brazzaville` time zone.
		pub struct Brazzaville;
		impl Sealed for Brazzaville {
			const NAME: &'static str = "Africa/Brazzaville";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Brazzaville {}
		/// The `Africa/Bujumbura` time zone.
		pub struct Bujumbura;
		impl Sealed for Bujumbura {
			const NAME: &'static str = "Africa/Bujumbura";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Bujumbura {}
		/// The `Africa/Cairo` time zone.
		pub struct Cairo;
		impl Sealed for Cairo {
			const NAME: &'static str = "Africa/Cairo";
			const DATA: TzifData = EGYPT;
		}
		impl TimeZone for Cairo {}
		/// The `Africa/Casablanca` time zone.
		pub struct Casablanca;
		impl Sealed for Casablanca {
			const NAME: &'static str = "Africa/Casablanca";
			const DATA: TzifData = AFRICA_CASABLANCA;
		}
		impl TimeZone for Casablanca {}
		/// The `Africa/Ceuta` time zone.
		pub struct Ceuta;
		impl Sealed for Ceuta {
			const NAME: &'static str = "Africa/Ceuta";
			const DATA: TzifData = AFRICA_CEUTA;
		}
		impl TimeZone for Ceuta {}
		/// The `Africa/Conakry` time zone.
		pub struct Conakry;
		impl Sealed for Conakry {
			const NAME: &'static str = "Africa/Conakry";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Conakry {}
		/// The `Africa/Dakar` time zone.
		pub struct Dakar;
		impl Sealed for Dakar {
			const NAME: &'static str = "Africa/Dakar";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Dakar {}
		/// The `Africa/Dar_es_Salaam` time zone.
		pub struct DarEsSalaam;
		impl Sealed for DarEsSalaam {
			const NAME: &'static str = "Africa/Dar_es_Salaam";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for DarEsSalaam {}
		/// The `Africa/Djibouti` time zone.
		pub struct Djibouti;
		impl Sealed for Djibouti {
			const NAME: &'static str = "Africa/Djibouti";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Djibouti {}
		/// The `Africa/Douala` time zone.
		pub struct Douala;
		impl Sealed for Douala {
			const NAME: &'static str = "Africa/Douala";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Douala {}
		/// The `Africa/El_Aaiun` time zone.
		pub struct ElAaiun;
		impl Sealed for ElAaiun {
			const NAME: &'static str = "Africa/El_Aaiun";
			const DATA: TzifData = AFRICA_EL_AAIUN;
		}
		impl TimeZone for ElAaiun {}
		/// The `Africa/Freetown` time zone.
		pub struct Freetown;
		impl Sealed for Freetown {
			const NAME: &'static str = "Africa/Freetown";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Freetown {}
		/// The `Africa/Gaborone` time zone.
		pub struct Gaborone;
		impl Sealed for Gaborone {
			const NAME: &'static str = "Africa/Gaborone";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Gaborone {}
		/// The `Africa/Harare` time zone.
		pub struct Harare;
		impl Sealed for Harare {
			const NAME: &'static str = "Africa/Harare";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Harare {}
		/// The `Africa/Johannesburg` time zone.
		pub struct Johannesburg;
		impl Sealed for Johannesburg {
			const NAME: &'static str = "Africa/Johannesburg";
			const DATA: TzifData = AFRICA_MBABANE;
		}
		impl TimeZone for Johannesburg {}
		/// The `Africa/Juba` time zone.
		pub struct Juba;
		impl Sealed for Juba {
			const NAME: &'static str = "Africa/Juba";
			const DATA: TzifData = AFRICA_JUBA;
		}
		impl TimeZone for Juba {}
		/// The `Africa/Kampala` time zone.
		pub struct Kampala;
		impl Sealed for Kampala {
			const NAME: &'static str = "Africa/Kampala";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Kampala {}
		/// The `Africa/Khartoum` time zone.
		pub struct Khartoum;
		impl Sealed for Khartoum {
			const NAME: &'static str = "Africa/Khartoum";
			const DATA: TzifData = AFRICA_KHARTOUM;
		}
		impl TimeZone for Khartoum {}
		/// The `Africa/Kigali` time zone.
		pub struct Kigali;
		impl Sealed for Kigali {
			const NAME: &'static str = "Africa/Kigali";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Kigali {}
		/// The `Africa/Kinshasa` time zone.
		pub struct Kinshasa;
		impl Sealed for Kinshasa {
			const NAME: &'static str = "Africa/Kinshasa";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Kinshasa {}
		/// The `Africa/Lagos` time zone.
		pub struct Lagos;
		impl Sealed for Lagos {
			const NAME: &'static str = "Africa/Lagos";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Lagos {}
		/// The `Africa/Libreville` time zone.
		pub struct Libreville;
		impl Sealed for Libreville {
			const NAME: &'static str = "Africa/Libreville";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Libreville {}
		/// The `Africa/Lome` time zone.
		pub struct Lome;
		impl Sealed for Lome {
			const NAME: &'static str = "Africa/Lome";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Lome {}
		/// The `Africa/Luanda` time zone.
		pub struct Luanda;
		impl Sealed for Luanda {
			const NAME: &'static str = "Africa/Luanda";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Luanda {}
		/// The `Africa/Lubumbashi` time zone.
		pub struct Lubumbashi;
		impl Sealed for Lubumbashi {
			const NAME: &'static str = "Africa/Lubumbashi";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Lubumbashi {}
		/// The `Africa/Lusaka` time zone.
		pub struct Lusaka;
		impl Sealed for Lusaka {
			const NAME: &'static str = "Africa/Lusaka";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Lusaka {}
		/// The `Africa/Malabo` time zone.
		pub struct Malabo;
		impl Sealed for Malabo {
			const NAME: &'static str = "Africa/Malabo";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Malabo {}
		/// The `Africa/Maputo` time zone.
		pub struct Maputo;
		impl Sealed for Maputo {
			const NAME: &'static str = "Africa/Maputo";
			const DATA: TzifData = AFRICA_LUSAKA;
		}
		impl TimeZone for Maputo {}
		/// The `Africa/Maseru` time zone.
		pub struct Maseru;
		impl Sealed for Maseru {
			const NAME: &'static str = "Africa/Maseru";
			const DATA: TzifData = AFRICA_MBABANE;
		}
		impl TimeZone for Maseru {}
		/// The `Africa/Mbabane` time zone.
		pub struct Mbabane;
		impl Sealed for Mbabane {
			const NAME: &'static str = "Africa/Mbabane";
			const DATA: TzifData = AFRICA_MBABANE;
		}
		impl TimeZone for Mbabane {}
		/// The `Africa/Mogadishu` time zone.
		pub struct Mogadishu;
		impl Sealed for Mogadishu {
			const NAME: &'static str = "Africa/Mogadishu";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Mogadishu {}
		/// The `Africa/Monrovia` time zone.
		pub struct Monrovia;
		impl Sealed for Monrovia {
			const NAME: &'static str = "Africa/Monrovia";
			const DATA: TzifData = AFRICA_MONROVIA;
		}
		impl TimeZone for Monrovia {}
		/// The `Africa/Nairobi` time zone.
		pub struct Nairobi;
		impl Sealed for Nairobi {
			const NAME: &'static str = "Africa/Nairobi";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Nairobi {}
		/// The `Africa/Ndjamena` time zone.
		pub struct Ndjamena;
		impl Sealed for Ndjamena {
			const NAME: &'static str = "Africa/Ndjamena";
			const DATA: TzifData = AFRICA_NDJAMENA;
		}
		impl TimeZone for Ndjamena {}
		/// The `Africa/Niamey` time zone.
		pub struct Niamey;
		impl Sealed for Niamey {
			const NAME: &'static str = "Africa/Niamey";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for Niamey {}
		/// The `Africa/Nouakchott` time zone.
		pub struct Nouakchott;
		impl Sealed for Nouakchott {
			const NAME: &'static str = "Africa/Nouakchott";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Nouakchott {}
		/// The `Africa/Ouagadougou` time zone.
		pub struct Ouagadougou;
		impl Sealed for Ouagadougou {
			const NAME: &'static str = "Africa/Ouagadougou";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Ouagadougou {}
		/// The `Africa/Porto-Novo` time zone.
		pub struct PortoNovo;
		impl Sealed for PortoNovo {
			const NAME: &'static str = "Africa/Porto-Novo";
			const DATA: TzifData = AFRICA_PORTO_NOVO;
		}
		impl TimeZone for PortoNovo {}
		/// The `Africa/Sao_Tome` time zone.
		pub struct SaoTome;
		impl Sealed for SaoTome {
			const NAME: &'static str = "Africa/Sao_Tome";
			const DATA: TzifData = AFRICA_SAO_TOME;
		}
		impl TimeZone for SaoTome {}
		/// The `Africa/Timbuktu` time zone.
		pub struct Timbuktu;
		impl Sealed for Timbuktu {
			const NAME: &'static str = "Africa/Timbuktu";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Timbuktu {}
		/// The `Africa/Tripoli` time zone.
		pub struct Tripoli;
		impl Sealed for Tripoli {
			const NAME: &'static str = "Africa/Tripoli";
			const DATA: TzifData = LIBYA;
		}
		impl TimeZone for Tripoli {}
		/// The `Africa/Tunis` time zone.
		pub struct Tunis;
		impl Sealed for Tunis {
			const NAME: &'static str = "Africa/Tunis";
			const DATA: TzifData = AFRICA_TUNIS;
		}
		impl TimeZone for Tunis {}
		/// The `Africa/Windhoek` time zone.
		pub struct Windhoek;
		impl Sealed for Windhoek {
			const NAME: &'static str = "Africa/Windhoek";
			const DATA: TzifData = AFRICA_WINDHOEK;
		}
		impl TimeZone for Windhoek {}
	}
	pub mod America {
		use super::*;
		/// The `America/Adak` time zone.
		pub struct Adak;
		impl Sealed for Adak {
			const NAME: &'static str = "America/Adak";
			const DATA: TzifData = US_ALEUTIAN;
		}
		impl TimeZone for Adak {}
		/// The `America/Anchorage` time zone.
		pub struct Anchorage;
		impl Sealed for Anchorage {
			const NAME: &'static str = "America/Anchorage";
			const DATA: TzifData = US_ALASKA;
		}
		impl TimeZone for Anchorage {}
		/// The `America/Anguilla` time zone.
		pub struct Anguilla;
		impl Sealed for Anguilla {
			const NAME: &'static str = "America/Anguilla";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Anguilla {}
		/// The `America/Antigua` time zone.
		pub struct Antigua;
		impl Sealed for Antigua {
			const NAME: &'static str = "America/Antigua";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Antigua {}
		/// The `America/Araguaina` time zone.
		pub struct Araguaina;
		impl Sealed for Araguaina {
			const NAME: &'static str = "America/Araguaina";
			const DATA: TzifData = AMERICA_ARAGUAINA;
		}
		impl TimeZone for Araguaina {}
		pub mod Argentina {
			use super::*;
			/// The `America/Argentina/Buenos_Aires` time zone.
			pub struct BuenosAires;
			impl Sealed for BuenosAires {
				const NAME: &'static str = "America/Argentina/Buenos_Aires";
				const DATA: TzifData = AMERICA_BUENOS_AIRES;
			}
			impl TimeZone for BuenosAires {}
			/// The `America/Argentina/Catamarca` time zone.
			pub struct Catamarca;
			impl Sealed for Catamarca {
				const NAME: &'static str = "America/Argentina/Catamarca";
				const DATA: TzifData = AMERICA_CATAMARCA;
			}
			impl TimeZone for Catamarca {}
			/// The `America/Argentina/ComodRivadavia` time zone.
			pub struct ComodRivadavia;
			impl Sealed for ComodRivadavia {
				const NAME: &'static str = "America/Argentina/ComodRivadavia";
				const DATA: TzifData = AMERICA_CATAMARCA;
			}
			impl TimeZone for ComodRivadavia {}
			/// The `America/Argentina/Cordoba` time zone.
			pub struct Cordoba;
			impl Sealed for Cordoba {
				const NAME: &'static str = "America/Argentina/Cordoba";
				const DATA: TzifData = AMERICA_ROSARIO;
			}
			impl TimeZone for Cordoba {}
			/// The `America/Argentina/Jujuy` time zone.
			pub struct Jujuy;
			impl Sealed for Jujuy {
				const NAME: &'static str = "America/Argentina/Jujuy";
				const DATA: TzifData = AMERICA_JUJUY;
			}
			impl TimeZone for Jujuy {}
			/// The `America/Argentina/La_Rioja` time zone.
			pub struct LaRioja;
			impl Sealed for LaRioja {
				const NAME: &'static str = "America/Argentina/La_Rioja";
				const DATA: TzifData = AMERICA_ARGENTINA_LA_RIOJA;
			}
			impl TimeZone for LaRioja {}
			/// The `America/Argentina/Mendoza` time zone.
			pub struct Mendoza;
			impl Sealed for Mendoza {
				const NAME: &'static str = "America/Argentina/Mendoza";
				const DATA: TzifData = AMERICA_MENDOZA;
			}
			impl TimeZone for Mendoza {}
			/// The `America/Argentina/Rio_Gallegos` time zone.
			pub struct RioGallegos;
			impl Sealed for RioGallegos {
				const NAME: &'static str = "America/Argentina/Rio_Gallegos";
				const DATA: TzifData = AMERICA_ARGENTINA_RIO_GALLEGOS;
			}
			impl TimeZone for RioGallegos {}
			/// The `America/Argentina/Salta` time zone.
			pub struct Salta;
			impl Sealed for Salta {
				const NAME: &'static str = "America/Argentina/Salta";
				const DATA: TzifData = AMERICA_ARGENTINA_SALTA;
			}
			impl TimeZone for Salta {}
			/// The `America/Argentina/San_Juan` time zone.
			pub struct SanJuan;
			impl Sealed for SanJuan {
				const NAME: &'static str = "America/Argentina/San_Juan";
				const DATA: TzifData = AMERICA_ARGENTINA_SAN_JUAN;
			}
			impl TimeZone for SanJuan {}
			/// The `America/Argentina/San_Luis` time zone.
			pub struct SanLuis;
			impl Sealed for SanLuis {
				const NAME: &'static str = "America/Argentina/San_Luis";
				const DATA: TzifData = AMERICA_ARGENTINA_SAN_LUIS;
			}
			impl TimeZone for SanLuis {}
			/// The `America/Argentina/Tucuman` time zone.
			pub struct Tucuman;
			impl Sealed for Tucuman {
				const NAME: &'static str = "America/Argentina/Tucuman";
				const DATA: TzifData = AMERICA_ARGENTINA_TUCUMAN;
			}
			impl TimeZone for Tucuman {}
			/// The `America/Argentina/Ushuaia` time zone.
			pub struct Ushuaia;
			impl Sealed for Ushuaia {
				const NAME: &'static str = "America/Argentina/Ushuaia";
				const DATA: TzifData = AMERICA_ARGENTINA_USHUAIA;
			}
			impl TimeZone for Ushuaia {}
		}
		/// The `America/Aruba` time zone.
		pub struct Aruba;
		impl Sealed for Aruba {
			const NAME: &'static str = "America/Aruba";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Aruba {}
		/// The `America/Asuncion` time zone.
		pub struct Asuncion;
		impl Sealed for Asuncion {
			const NAME: &'static str = "America/Asuncion";
			const DATA: TzifData = AMERICA_ASUNCION;
		}
		impl TimeZone for Asuncion {}
		/// The `America/Atikokan` time zone.
		pub struct Atikokan;
		impl Sealed for Atikokan {
			const NAME: &'static str = "America/Atikokan";
			const DATA: TzifData = AMERICA_CORAL_HARBOUR;
		}
		impl TimeZone for Atikokan {}
		/// The `America/Atka` time zone.
		pub struct Atka;
		impl Sealed for Atka {
			const NAME: &'static str = "America/Atka";
			const DATA: TzifData = US_ALEUTIAN;
		}
		impl TimeZone for Atka {}
		/// The `America/Bahia` time zone.
		pub struct Bahia;
		impl Sealed for Bahia {
			const NAME: &'static str = "America/Bahia";
			const DATA: TzifData = AMERICA_BAHIA;
		}
		impl TimeZone for Bahia {}
		/// The `America/Bahia_Banderas` time zone.
		pub struct BahiaBanderas;
		impl Sealed for BahiaBanderas {
			const NAME: &'static str = "America/Bahia_Banderas";
			const DATA: TzifData = AMERICA_BAHIA_BANDERAS;
		}
		impl TimeZone for BahiaBanderas {}
		/// The `America/Barbados` time zone.
		pub struct Barbados;
		impl Sealed for Barbados {
			const NAME: &'static str = "America/Barbados";
			const DATA: TzifData = AMERICA_BARBADOS;
		}
		impl TimeZone for Barbados {}
		/// The `America/Belem` time zone.
		pub struct Belem;
		impl Sealed for Belem {
			const NAME: &'static str = "America/Belem";
			const DATA: TzifData = AMERICA_BELEM;
		}
		impl TimeZone for Belem {}
		/// The `America/Belize` time zone.
		pub struct Belize;
		impl Sealed for Belize {
			const NAME: &'static str = "America/Belize";
			const DATA: TzifData = AMERICA_BELIZE;
		}
		impl TimeZone for Belize {}
		/// The `America/Blanc-Sablon` time zone.
		pub struct BlancSablon;
		impl Sealed for BlancSablon {
			const NAME: &'static str = "America/Blanc-Sablon";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for BlancSablon {}
		/// The `America/Boa_Vista` time zone.
		pub struct BoaVista;
		impl Sealed for BoaVista {
			const NAME: &'static str = "America/Boa_Vista";
			const DATA: TzifData = AMERICA_BOA_VISTA;
		}
		impl TimeZone for BoaVista {}
		/// The `America/Bogota` time zone.
		pub struct Bogota;
		impl Sealed for Bogota {
			const NAME: &'static str = "America/Bogota";
			const DATA: TzifData = AMERICA_BOGOTA;
		}
		impl TimeZone for Bogota {}
		/// The `America/Boise` time zone.
		pub struct Boise;
		impl Sealed for Boise {
			const NAME: &'static str = "America/Boise";
			const DATA: TzifData = AMERICA_BOISE;
		}
		impl TimeZone for Boise {}
		/// The `America/Buenos_Aires` time zone.
		pub struct BuenosAires;
		impl Sealed for BuenosAires {
			const NAME: &'static str = "America/Buenos_Aires";
			const DATA: TzifData = AMERICA_BUENOS_AIRES;
		}
		impl TimeZone for BuenosAires {}
		/// The `America/Cambridge_Bay` time zone.
		pub struct CambridgeBay;
		impl Sealed for CambridgeBay {
			const NAME: &'static str = "America/Cambridge_Bay";
			const DATA: TzifData = AMERICA_CAMBRIDGE_BAY;
		}
		impl TimeZone for CambridgeBay {}
		/// The `America/Campo_Grande` time zone.
		pub struct CampoGrande;
		impl Sealed for CampoGrande {
			const NAME: &'static str = "America/Campo_Grande";
			const DATA: TzifData = AMERICA_CAMPO_GRANDE;
		}
		impl TimeZone for CampoGrande {}
		/// The `America/Cancun` time zone.
		pub struct Cancun;
		impl Sealed for Cancun {
			const NAME: &'static str = "America/Cancun";
			const DATA: TzifData = AMERICA_CANCUN;
		}
		impl TimeZone for Cancun {}
		/// The `America/Caracas` time zone.
		pub struct Caracas;
		impl Sealed for Caracas {
			const NAME: &'static str = "America/Caracas";
			const DATA: TzifData = AMERICA_CARACAS;
		}
		impl TimeZone for Caracas {}
		/// The `America/Catamarca` time zone.
		pub struct Catamarca;
		impl Sealed for Catamarca {
			const NAME: &'static str = "America/Catamarca";
			const DATA: TzifData = AMERICA_CATAMARCA;
		}
		impl TimeZone for Catamarca {}
		/// The `America/Cayenne` time zone.
		pub struct Cayenne;
		impl Sealed for Cayenne {
			const NAME: &'static str = "America/Cayenne";
			const DATA: TzifData = AMERICA_CAYENNE;
		}
		impl TimeZone for Cayenne {}
		/// The `America/Cayman` time zone.
		pub struct Cayman;
		impl Sealed for Cayman {
			const NAME: &'static str = "America/Cayman";
			const DATA: TzifData = AMERICA_CORAL_HARBOUR;
		}
		impl TimeZone for Cayman {}
		/// The `America/Chicago` time zone.
		pub struct Chicago;
		impl Sealed for Chicago {
			const NAME: &'static str = "America/Chicago";
			const DATA: TzifData = US_CENTRAL;
		}
		impl TimeZone for Chicago {}
		/// The `America/Chihuahua` time zone.
		pub struct Chihuahua;
		impl Sealed for Chihuahua {
			const NAME: &'static str = "America/Chihuahua";
			const DATA: TzifData = AMERICA_CHIHUAHUA;
		}
		impl TimeZone for Chihuahua {}
		/// The `America/Ciudad_Juarez` time zone.
		pub struct CiudadJuarez;
		impl Sealed for CiudadJuarez {
			const NAME: &'static str = "America/Ciudad_Juarez";
			const DATA: TzifData = AMERICA_CIUDAD_JUAREZ;
		}
		impl TimeZone for CiudadJuarez {}
		/// The `America/Coral_Harbour` time zone.
		pub struct CoralHarbour;
		impl Sealed for CoralHarbour {
			const NAME: &'static str = "America/Coral_Harbour";
			const DATA: TzifData = AMERICA_CORAL_HARBOUR;
		}
		impl TimeZone for CoralHarbour {}
		/// The `America/Cordoba` time zone.
		pub struct Cordoba;
		impl Sealed for Cordoba {
			const NAME: &'static str = "America/Cordoba";
			const DATA: TzifData = AMERICA_ROSARIO;
		}
		impl TimeZone for Cordoba {}
		/// The `America/Costa_Rica` time zone.
		pub struct CostaRica;
		impl Sealed for CostaRica {
			const NAME: &'static str = "America/Costa_Rica";
			const DATA: TzifData = AMERICA_COSTA_RICA;
		}
		impl TimeZone for CostaRica {}
		/// The `America/Creston` time zone.
		pub struct Creston;
		impl Sealed for Creston {
			const NAME: &'static str = "America/Creston";
			const DATA: TzifData = US_ARIZONA;
		}
		impl TimeZone for Creston {}
		/// The `America/Cuiaba` time zone.
		pub struct Cuiaba;
		impl Sealed for Cuiaba {
			const NAME: &'static str = "America/Cuiaba";
			const DATA: TzifData = AMERICA_CUIABA;
		}
		impl TimeZone for Cuiaba {}
		/// The `America/Curacao` time zone.
		pub struct Curacao;
		impl Sealed for Curacao {
			const NAME: &'static str = "America/Curacao";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Curacao {}
		/// The `America/Danmarkshavn` time zone.
		pub struct Danmarkshavn;
		impl Sealed for Danmarkshavn {
			const NAME: &'static str = "America/Danmarkshavn";
			const DATA: TzifData = AMERICA_DANMARKSHAVN;
		}
		impl TimeZone for Danmarkshavn {}
		/// The `America/Dawson` time zone.
		pub struct Dawson;
		impl Sealed for Dawson {
			const NAME: &'static str = "America/Dawson";
			const DATA: TzifData = AMERICA_DAWSON;
		}
		impl TimeZone for Dawson {}
		/// The `America/Dawson_Creek` time zone.
		pub struct DawsonCreek;
		impl Sealed for DawsonCreek {
			const NAME: &'static str = "America/Dawson_Creek";
			const DATA: TzifData = AMERICA_DAWSON_CREEK;
		}
		impl TimeZone for DawsonCreek {}
		/// The `America/Denver` time zone.
		pub struct Denver;
		impl Sealed for Denver {
			const NAME: &'static str = "America/Denver";
			const DATA: TzifData = US_MOUNTAIN;
		}
		impl TimeZone for Denver {}
		/// The `America/Detroit` time zone.
		pub struct Detroit;
		impl Sealed for Detroit {
			const NAME: &'static str = "America/Detroit";
			const DATA: TzifData = US_MICHIGAN;
		}
		impl TimeZone for Detroit {}
		/// The `America/Dominica` time zone.
		pub struct Dominica;
		impl Sealed for Dominica {
			const NAME: &'static str = "America/Dominica";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Dominica {}
		/// The `America/Edmonton` time zone.
		pub struct Edmonton;
		impl Sealed for Edmonton {
			const NAME: &'static str = "America/Edmonton";
			const DATA: TzifData = CANADA_MOUNTAIN;
		}
		impl TimeZone for Edmonton {}
		/// The `America/Eirunepe` time zone.
		pub struct Eirunepe;
		impl Sealed for Eirunepe {
			const NAME: &'static str = "America/Eirunepe";
			const DATA: TzifData = AMERICA_EIRUNEPE;
		}
		impl TimeZone for Eirunepe {}
		/// The `America/El_Salvador` time zone.
		pub struct ElSalvador;
		impl Sealed for ElSalvador {
			const NAME: &'static str = "America/El_Salvador";
			const DATA: TzifData = AMERICA_EL_SALVADOR;
		}
		impl TimeZone for ElSalvador {}
		/// The `America/Ensenada` time zone.
		pub struct Ensenada;
		impl Sealed for Ensenada {
			const NAME: &'static str = "America/Ensenada";
			const DATA: TzifData = MEXICO_BAJA_NORTE;
		}
		impl TimeZone for Ensenada {}
		/// The `America/Fort_Nelson` time zone.
		pub struct FortNelson;
		impl Sealed for FortNelson {
			const NAME: &'static str = "America/Fort_Nelson";
			const DATA: TzifData = AMERICA_FORT_NELSON;
		}
		impl TimeZone for FortNelson {}
		/// The `America/Fort_Wayne` time zone.
		pub struct FortWayne;
		impl Sealed for FortWayne {
			const NAME: &'static str = "America/Fort_Wayne";
			const DATA: TzifData = US_EAST_INDIANA;
		}
		impl TimeZone for FortWayne {}
		/// The `America/Fortaleza` time zone.
		pub struct Fortaleza;
		impl Sealed for Fortaleza {
			const NAME: &'static str = "America/Fortaleza";
			const DATA: TzifData = AMERICA_FORTALEZA;
		}
		impl TimeZone for Fortaleza {}
		/// The `America/Glace_Bay` time zone.
		pub struct GlaceBay;
		impl Sealed for GlaceBay {
			const NAME: &'static str = "America/Glace_Bay";
			const DATA: TzifData = AMERICA_GLACE_BAY;
		}
		impl TimeZone for GlaceBay {}
		/// The `America/Godthab` time zone.
		pub struct Godthab;
		impl Sealed for Godthab {
			const NAME: &'static str = "America/Godthab";
			const DATA: TzifData = AMERICA_GODTHAB;
		}
		impl TimeZone for Godthab {}
		/// The `America/Goose_Bay` time zone.
		pub struct GooseBay;
		impl Sealed for GooseBay {
			const NAME: &'static str = "America/Goose_Bay";
			const DATA: TzifData = AMERICA_GOOSE_BAY;
		}
		impl TimeZone for GooseBay {}
		/// The `America/Grand_Turk` time zone.
		pub struct GrandTurk;
		impl Sealed for GrandTurk {
			const NAME: &'static str = "America/Grand_Turk";
			const DATA: TzifData = AMERICA_GRAND_TURK;
		}
		impl TimeZone for GrandTurk {}
		/// The `America/Grenada` time zone.
		pub struct Grenada;
		impl Sealed for Grenada {
			const NAME: &'static str = "America/Grenada";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Grenada {}
		/// The `America/Guadeloupe` time zone.
		pub struct Guadeloupe;
		impl Sealed for Guadeloupe {
			const NAME: &'static str = "America/Guadeloupe";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Guadeloupe {}
		/// The `America/Guatemala` time zone.
		pub struct Guatemala;
		impl Sealed for Guatemala {
			const NAME: &'static str = "America/Guatemala";
			const DATA: TzifData = AMERICA_GUATEMALA;
		}
		impl TimeZone for Guatemala {}
		/// The `America/Guayaquil` time zone.
		pub struct Guayaquil;
		impl Sealed for Guayaquil {
			const NAME: &'static str = "America/Guayaquil";
			const DATA: TzifData = AMERICA_GUAYAQUIL;
		}
		impl TimeZone for Guayaquil {}
		/// The `America/Guyana` time zone.
		pub struct Guyana;
		impl Sealed for Guyana {
			const NAME: &'static str = "America/Guyana";
			const DATA: TzifData = AMERICA_GUYANA;
		}
		impl TimeZone for Guyana {}
		/// The `America/Halifax` time zone.
		pub struct Halifax;
		impl Sealed for Halifax {
			const NAME: &'static str = "America/Halifax";
			const DATA: TzifData = CANADA_ATLANTIC;
		}
		impl TimeZone for Halifax {}
		/// The `America/Havana` time zone.
		pub struct Havana;
		impl Sealed for Havana {
			const NAME: &'static str = "America/Havana";
			const DATA: TzifData = CUBA;
		}
		impl TimeZone for Havana {}
		/// The `America/Hermosillo` time zone.
		pub struct Hermosillo;
		impl Sealed for Hermosillo {
			const NAME: &'static str = "America/Hermosillo";
			const DATA: TzifData = AMERICA_HERMOSILLO;
		}
		impl TimeZone for Hermosillo {}
		pub mod Indiana {
			use super::*;
			/// The `America/Indiana/Indianapolis` time zone.
			pub struct Indianapolis;
			impl Sealed for Indianapolis {
				const NAME: &'static str = "America/Indiana/Indianapolis";
				const DATA: TzifData = US_EAST_INDIANA;
			}
			impl TimeZone for Indianapolis {}
			/// The `America/Indiana/Knox` time zone.
			pub struct Knox;
			impl Sealed for Knox {
				const NAME: &'static str = "America/Indiana/Knox";
				const DATA: TzifData = US_INDIANA_STARKE;
			}
			impl TimeZone for Knox {}
			/// The `America/Indiana/Marengo` time zone.
			pub struct Marengo;
			impl Sealed for Marengo {
				const NAME: &'static str = "America/Indiana/Marengo";
				const DATA: TzifData = AMERICA_INDIANA_MARENGO;
			}
			impl TimeZone for Marengo {}
			/// The `America/Indiana/Petersburg` time zone.
			pub struct Petersburg;
			impl Sealed for Petersburg {
				const NAME: &'static str = "America/Indiana/Petersburg";
				const DATA: TzifData = AMERICA_INDIANA_PETERSBURG;
			}
			impl TimeZone for Petersburg {}
			/// The `America/Indiana/Tell_City` time zone.
			pub struct TellCity;
			impl Sealed for TellCity {
				const NAME: &'static str = "America/Indiana/Tell_City";
				const DATA: TzifData = AMERICA_INDIANA_TELL_CITY;
			}
			impl TimeZone for TellCity {}
			/// The `America/Indiana/Vevay` time zone.
			pub struct Vevay;
			impl Sealed for Vevay {
				const NAME: &'static str = "America/Indiana/Vevay";
				const DATA: TzifData = AMERICA_INDIANA_VEVAY;
			}
			impl TimeZone for Vevay {}
			/// The `America/Indiana/Vincennes` time zone.
			pub struct Vincennes;
			impl Sealed for Vincennes {
				const NAME: &'static str = "America/Indiana/Vincennes";
				const DATA: TzifData = AMERICA_INDIANA_VINCENNES;
			}
			impl TimeZone for Vincennes {}
			/// The `America/Indiana/Winamac` time zone.
			pub struct Winamac;
			impl Sealed for Winamac {
				const NAME: &'static str = "America/Indiana/Winamac";
				const DATA: TzifData = AMERICA_INDIANA_WINAMAC;
			}
			impl TimeZone for Winamac {}
		}
		/// The `America/Indianapolis` time zone.
		pub struct Indianapolis;
		impl Sealed for Indianapolis {
			const NAME: &'static str = "America/Indianapolis";
			const DATA: TzifData = US_EAST_INDIANA;
		}
		impl TimeZone for Indianapolis {}
		/// The `America/Inuvik` time zone.
		pub struct Inuvik;
		impl Sealed for Inuvik {
			const NAME: &'static str = "America/Inuvik";
			const DATA: TzifData = AMERICA_INUVIK;
		}
		impl TimeZone for Inuvik {}
		/// The `America/Iqaluit` time zone.
		pub struct Iqaluit;
		impl Sealed for Iqaluit {
			const NAME: &'static str = "America/Iqaluit";
			const DATA: TzifData = AMERICA_PANGNIRTUNG;
		}
		impl TimeZone for Iqaluit {}
		/// The `America/Jamaica` time zone.
		pub struct Jamaica;
		impl Sealed for Jamaica {
			const NAME: &'static str = "America/Jamaica";
			const DATA: TzifData = JAMAICA;
		}
		impl TimeZone for Jamaica {}
		/// The `America/Jujuy` time zone.
		pub struct Jujuy;
		impl Sealed for Jujuy {
			const NAME: &'static str = "America/Jujuy";
			const DATA: TzifData = AMERICA_JUJUY;
		}
		impl TimeZone for Jujuy {}
		/// The `America/Juneau` time zone.
		pub struct Juneau;
		impl Sealed for Juneau {
			const NAME: &'static str = "America/Juneau";
			const DATA: TzifData = AMERICA_JUNEAU;
		}
		impl TimeZone for Juneau {}
		pub mod Kentucky {
			use super::*;
			/// The `America/Kentucky/Louisville` time zone.
			pub struct Louisville;
			impl Sealed for Louisville {
				const NAME: &'static str = "America/Kentucky/Louisville";
				const DATA: TzifData = AMERICA_LOUISVILLE;
			}
			impl TimeZone for Louisville {}
			/// The `America/Kentucky/Monticello` time zone.
			pub struct Monticello;
			impl Sealed for Monticello {
				const NAME: &'static str = "America/Kentucky/Monticello";
				const DATA: TzifData = AMERICA_KENTUCKY_MONTICELLO;
			}
			impl TimeZone for Monticello {}
		}
		/// The `America/Knox_IN` time zone.
		pub struct KnoxIn;
		impl Sealed for KnoxIn {
			const NAME: &'static str = "America/Knox_IN";
			const DATA: TzifData = US_INDIANA_STARKE;
		}
		impl TimeZone for KnoxIn {}
		/// The `America/Kralendijk` time zone.
		pub struct Kralendijk;
		impl Sealed for Kralendijk {
			const NAME: &'static str = "America/Kralendijk";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Kralendijk {}
		/// The `America/La_Paz` time zone.
		pub struct LaPaz;
		impl Sealed for LaPaz {
			const NAME: &'static str = "America/La_Paz";
			const DATA: TzifData = AMERICA_LA_PAZ;
		}
		impl TimeZone for LaPaz {}
		/// The `America/Lima` time zone.
		pub struct Lima;
		impl Sealed for Lima {
			const NAME: &'static str = "America/Lima";
			const DATA: TzifData = AMERICA_LIMA;
		}
		impl TimeZone for Lima {}
		/// The `America/Los_Angeles` time zone.
		pub struct LosAngeles;
		impl Sealed for LosAngeles {
			const NAME: &'static str = "America/Los_Angeles";
			const DATA: TzifData = US_PACIFIC;
		}
		impl TimeZone for LosAngeles {}
		/// The `America/Louisville` time zone.
		pub struct Louisville;
		impl Sealed for Louisville {
			const NAME: &'static str = "America/Louisville";
			const DATA: TzifData = AMERICA_LOUISVILLE;
		}
		impl TimeZone for Louisville {}
		/// The `America/Lower_Princes` time zone.
		pub struct LowerPrinces;
		impl Sealed for LowerPrinces {
			const NAME: &'static str = "America/Lower_Princes";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for LowerPrinces {}
		/// The `America/Maceio` time zone.
		pub struct Maceio;
		impl Sealed for Maceio {
			const NAME: &'static str = "America/Maceio";
			const DATA: TzifData = AMERICA_MACEIO;
		}
		impl TimeZone for Maceio {}
		/// The `America/Managua` time zone.
		pub struct Managua;
		impl Sealed for Managua {
			const NAME: &'static str = "America/Managua";
			const DATA: TzifData = AMERICA_MANAGUA;
		}
		impl TimeZone for Managua {}
		/// The `America/Manaus` time zone.
		pub struct Manaus;
		impl Sealed for Manaus {
			const NAME: &'static str = "America/Manaus";
			const DATA: TzifData = BRAZIL_WEST;
		}
		impl TimeZone for Manaus {}
		/// The `America/Marigot` time zone.
		pub struct Marigot;
		impl Sealed for Marigot {
			const NAME: &'static str = "America/Marigot";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Marigot {}
		/// The `America/Martinique` time zone.
		pub struct Martinique;
		impl Sealed for Martinique {
			const NAME: &'static str = "America/Martinique";
			const DATA: TzifData = AMERICA_MARTINIQUE;
		}
		impl TimeZone for Martinique {}
		/// The `America/Matamoros` time zone.
		pub struct Matamoros;
		impl Sealed for Matamoros {
			const NAME: &'static str = "America/Matamoros";
			const DATA: TzifData = AMERICA_MATAMOROS;
		}
		impl TimeZone for Matamoros {}
		/// The `America/Mazatlan` time zone.
		pub struct Mazatlan;
		impl Sealed for Mazatlan {
			const NAME: &'static str = "America/Mazatlan";
			const DATA: TzifData = MEXICO_BAJA_SUR;
		}
		impl TimeZone for Mazatlan {}
		/// The `America/Mendoza` time zone.
		pub struct Mendoza;
		impl Sealed for Mendoza {
			const NAME: &'static str = "America/Mendoza";
			const DATA: TzifData = AMERICA_MENDOZA;
		}
		impl TimeZone for Mendoza {}
		/// The `America/Menominee` time zone.
		pub struct Menominee;
		impl Sealed for Menominee {
			const NAME: &'static str = "America/Menominee";
			const DATA: TzifData = AMERICA_MENOMINEE;
		}
		impl TimeZone for Menominee {}
		/// The `America/Merida` time zone.
		pub struct Merida;
		impl Sealed for Merida {
			const NAME: &'static str = "America/Merida";
			const DATA: TzifData = AMERICA_MERIDA;
		}
		impl TimeZone for Merida {}
		/// The `America/Metlakatla` time zone.
		pub struct Metlakatla;
		impl Sealed for Metlakatla {
			const NAME: &'static str = "America/Metlakatla";
			const DATA: TzifData = AMERICA_METLAKATLA;
		}
		impl TimeZone for Metlakatla {}
		/// The `America/Mexico_City` time zone.
		pub struct MexicoCity;
		impl Sealed for MexicoCity {
			const NAME: &'static str = "America/Mexico_City";
			const DATA: TzifData = MEXICO_GENERAL;
		}
		impl TimeZone for MexicoCity {}
		/// The `America/Miquelon` time zone.
		pub struct Miquelon;
		impl Sealed for Miquelon {
			const NAME: &'static str = "America/Miquelon";
			const DATA: TzifData = AMERICA_MIQUELON;
		}
		impl TimeZone for Miquelon {}
		/// The `America/Moncton` time zone.
		pub struct Moncton;
		impl Sealed for Moncton {
			const NAME: &'static str = "America/Moncton";
			const DATA: TzifData = AMERICA_MONCTON;
		}
		impl TimeZone for Moncton {}
		/// The `America/Monterrey` time zone.
		pub struct Monterrey;
		impl Sealed for Monterrey {
			const NAME: &'static str = "America/Monterrey";
			const DATA: TzifData = AMERICA_MONTERREY;
		}
		impl TimeZone for Monterrey {}
		/// The `America/Montevideo` time zone.
		pub struct Montevideo;
		impl Sealed for Montevideo {
			const NAME: &'static str = "America/Montevideo";
			const DATA: TzifData = AMERICA_MONTEVIDEO;
		}
		impl TimeZone for Montevideo {}
		/// The `America/Montreal` time zone.
		pub struct Montreal;
		impl Sealed for Montreal {
			const NAME: &'static str = "America/Montreal";
			const DATA: TzifData = CANADA_EASTERN;
		}
		impl TimeZone for Montreal {}
		/// The `America/Montserrat` time zone.
		pub struct Montserrat;
		impl Sealed for Montserrat {
			const NAME: &'static str = "America/Montserrat";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Montserrat {}
		/// The `America/Nassau` time zone.
		pub struct Nassau;
		impl Sealed for Nassau {
			const NAME: &'static str = "America/Nassau";
			const DATA: TzifData = CANADA_EASTERN;
		}
		impl TimeZone for Nassau {}
		/// The `America/New_York` time zone.
		pub struct NewYork;
		impl Sealed for NewYork {
			const NAME: &'static str = "America/New_York";
			const DATA: TzifData = US_EASTERN;
		}
		impl TimeZone for NewYork {}
		/// The `America/Nipigon` time zone.
		pub struct Nipigon;
		impl Sealed for Nipigon {
			const NAME: &'static str = "America/Nipigon";
			const DATA: TzifData = CANADA_EASTERN;
		}
		impl TimeZone for Nipigon {}
		/// The `America/Nome` time zone.
		pub struct Nome;
		impl Sealed for Nome {
			const NAME: &'static str = "America/Nome";
			const DATA: TzifData = AMERICA_NOME;
		}
		impl TimeZone for Nome {}
		/// The `America/Noronha` time zone.
		pub struct Noronha;
		impl Sealed for Noronha {
			const NAME: &'static str = "America/Noronha";
			const DATA: TzifData = BRAZIL_DE_NORONHA;
		}
		impl TimeZone for Noronha {}
		pub mod NorthDakota {
			use super::*;
			/// The `America/North_Dakota/Beulah` time zone.
			pub struct Beulah;
			impl Sealed for Beulah {
				const NAME: &'static str = "America/North_Dakota/Beulah";
				const DATA: TzifData = AMERICA_NORTH_DAKOTA_BEULAH;
			}
			impl TimeZone for Beulah {}
			/// The `America/North_Dakota/Center` time zone.
			pub struct Center;
			impl Sealed for Center {
				const NAME: &'static str = "America/North_Dakota/Center";
				const DATA: TzifData = AMERICA_NORTH_DAKOTA_CENTER;
			}
			impl TimeZone for Center {}
			/// The `America/North_Dakota/New_Salem` time zone.
			pub struct NewSalem;
			impl Sealed for NewSalem {
				const NAME: &'static str = "America/North_Dakota/New_Salem";
				const DATA: TzifData = AMERICA_NORTH_DAKOTA_NEW_SALEM;
			}
			impl TimeZone for NewSalem {}
		}
		/// The `America/Nuuk` time zone.
		pub struct Nuuk;
		impl Sealed for Nuuk {
			const NAME: &'static str = "America/Nuuk";
			const DATA: TzifData = AMERICA_GODTHAB;
		}
		impl TimeZone for Nuuk {}
		/// The `America/Ojinaga` time zone.
		pub struct Ojinaga;
		impl Sealed for Ojinaga {
			const NAME: &'static str = "America/Ojinaga";
			const DATA: TzifData = AMERICA_OJINAGA;
		}
		impl TimeZone for Ojinaga {}
		/// The `America/Panama` time zone.
		pub struct Panama;
		impl Sealed for Panama {
			const NAME: &'static str = "America/Panama";
			const DATA: TzifData = AMERICA_CORAL_HARBOUR;
		}
		impl TimeZone for Panama {}
		/// The `America/Pangnirtung` time zone.
		pub struct Pangnirtung;
		impl Sealed for Pangnirtung {
			const NAME: &'static str = "America/Pangnirtung";
			const DATA: TzifData = AMERICA_PANGNIRTUNG;
		}
		impl TimeZone for Pangnirtung {}
		/// The `America/Paramaribo` time zone.
		pub struct Paramaribo;
		impl Sealed for Paramaribo {
			const NAME: &'static str = "America/Paramaribo";
			const DATA: TzifData = AMERICA_PARAMARIBO;
		}
		impl TimeZone for Paramaribo {}
		/// The `America/Phoenix` time zone.
		pub struct Phoenix;
		impl Sealed for Phoenix {
			const NAME: &'static str = "America/Phoenix";
			const DATA: TzifData = US_ARIZONA;
		}
		impl TimeZone for Phoenix {}
		/// The `America/Port-au-Prince` time zone.
		pub struct PortAuPrince;
		impl Sealed for PortAuPrince {
			const NAME: &'static str = "America/Port-au-Prince";
			const DATA: TzifData = AMERICA_PORT_AU_PRINCE;
		}
		impl TimeZone for PortAuPrince {}
		/// The `America/Port_of_Spain` time zone.
		pub struct PortOfSpain;
		impl Sealed for PortOfSpain {
			const NAME: &'static str = "America/Port_of_Spain";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for PortOfSpain {}
		/// The `America/Porto_Acre` time zone.
		pub struct PortoAcre;
		impl Sealed for PortoAcre {
			const NAME: &'static str = "America/Porto_Acre";
			const DATA: TzifData = BRAZIL_ACRE;
		}
		impl TimeZone for PortoAcre {}
		/// The `America/Porto_Velho` time zone.
		pub struct PortoVelho;
		impl Sealed for PortoVelho {
			const NAME: &'static str = "America/Porto_Velho";
			const DATA: TzifData = AMERICA_PORTO_VELHO;
		}
		impl TimeZone for PortoVelho {}
		/// The `America/Puerto_Rico` time zone.
		pub struct PuertoRico;
		impl Sealed for PuertoRico {
			const NAME: &'static str = "America/Puerto_Rico";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for PuertoRico {}
		/// The `America/Punta_Arenas` time zone.
		pub struct PuntaArenas;
		impl Sealed for PuntaArenas {
			const NAME: &'static str = "America/Punta_Arenas";
			const DATA: TzifData = AMERICA_PUNTA_ARENAS;
		}
		impl TimeZone for PuntaArenas {}
		/// The `America/Rainy_River` time zone.
		pub struct RainyRiver;
		impl Sealed for RainyRiver {
			const NAME: &'static str = "America/Rainy_River";
			const DATA: TzifData = CANADA_CENTRAL;
		}
		impl TimeZone for RainyRiver {}
		/// The `America/Rankin_Inlet` time zone.
		pub struct RankinInlet;
		impl Sealed for RankinInlet {
			const NAME: &'static str = "America/Rankin_Inlet";
			const DATA: TzifData = AMERICA_RANKIN_INLET;
		}
		impl TimeZone for RankinInlet {}
		/// The `America/Recife` time zone.
		pub struct Recife;
		impl Sealed for Recife {
			const NAME: &'static str = "America/Recife";
			const DATA: TzifData = AMERICA_RECIFE;
		}
		impl TimeZone for Recife {}
		/// The `America/Regina` time zone.
		pub struct Regina;
		impl Sealed for Regina {
			const NAME: &'static str = "America/Regina";
			const DATA: TzifData = CANADA_SASKATCHEWAN;
		}
		impl TimeZone for Regina {}
		/// The `America/Resolute` time zone.
		pub struct Resolute;
		impl Sealed for Resolute {
			const NAME: &'static str = "America/Resolute";
			const DATA: TzifData = AMERICA_RESOLUTE;
		}
		impl TimeZone for Resolute {}
		/// The `America/Rio_Branco` time zone.
		pub struct RioBranco;
		impl Sealed for RioBranco {
			const NAME: &'static str = "America/Rio_Branco";
			const DATA: TzifData = BRAZIL_ACRE;
		}
		impl TimeZone for RioBranco {}
		/// The `America/Rosario` time zone.
		pub struct Rosario;
		impl Sealed for Rosario {
			const NAME: &'static str = "America/Rosario";
			const DATA: TzifData = AMERICA_ROSARIO;
		}
		impl TimeZone for Rosario {}
		/// The `America/Santa_Isabel` time zone.
		pub struct SantaIsabel;
		impl Sealed for SantaIsabel {
			const NAME: &'static str = "America/Santa_Isabel";
			const DATA: TzifData = MEXICO_BAJA_NORTE;
		}
		impl TimeZone for SantaIsabel {}
		/// The `America/Santarem` time zone.
		pub struct Santarem;
		impl Sealed for Santarem {
			const NAME: &'static str = "America/Santarem";
			const DATA: TzifData = AMERICA_SANTAREM;
		}
		impl TimeZone for Santarem {}
		/// The `America/Santiago` time zone.
		pub struct Santiago;
		impl Sealed for Santiago {
			const NAME: &'static str = "America/Santiago";
			const DATA: TzifData = CHILE_CONTINENTAL;
		}
		impl TimeZone for Santiago {}
		/// The `America/Santo_Domingo` time zone.
		pub struct SantoDomingo;
		impl Sealed for SantoDomingo {
			const NAME: &'static str = "America/Santo_Domingo";
			const DATA: TzifData = AMERICA_SANTO_DOMINGO;
		}
		impl TimeZone for SantoDomingo {}
		/// The `America/Sao_Paulo` time zone.
		pub struct SaoPaulo;
		impl Sealed for SaoPaulo {
			const NAME: &'static str = "America/Sao_Paulo";
			const DATA: TzifData = BRAZIL_EAST;
		}
		impl TimeZone for SaoPaulo {}
		/// The `America/Scoresbysund` time zone.
		pub struct Scoresbysund;
		impl Sealed for Scoresbysund {
			const NAME: &'static str = "America/Scoresbysund";
			const DATA: TzifData = AMERICA_SCORESBYSUND;
		}
		impl TimeZone for Scoresbysund {}
		/// The `America/Shiprock` time zone.
		pub struct Shiprock;
		impl Sealed for Shiprock {
			const NAME: &'static str = "America/Shiprock";
			const DATA: TzifData = US_MOUNTAIN;
		}
		impl TimeZone for Shiprock {}
		/// The `America/Sitka` time zone.
		pub struct Sitka;
		impl Sealed for Sitka {
			const NAME: &'static str = "America/Sitka";
			const DATA: TzifData = AMERICA_SITKA;
		}
		impl TimeZone for Sitka {}
		/// The `America/St_Barthelemy` time zone.
		pub struct StBarthelemy;
		impl Sealed for StBarthelemy {
			const NAME: &'static str = "America/St_Barthelemy";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for StBarthelemy {}
		/// The `America/St_Johns` time zone.
		pub struct StJohns;
		impl Sealed for StJohns {
			const NAME: &'static str = "America/St_Johns";
			const DATA: TzifData = CANADA_NEWFOUNDLAND;
		}
		impl TimeZone for StJohns {}
		/// The `America/St_Kitts` time zone.
		pub struct StKitts;
		impl Sealed for StKitts {
			const NAME: &'static str = "America/St_Kitts";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for StKitts {}
		/// The `America/St_Lucia` time zone.
		pub struct StLucia;
		impl Sealed for StLucia {
			const NAME: &'static str = "America/St_Lucia";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for StLucia {}
		/// The `America/St_Thomas` time zone.
		pub struct StThomas;
		impl Sealed for StThomas {
			const NAME: &'static str = "America/St_Thomas";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for StThomas {}
		/// The `America/St_Vincent` time zone.
		pub struct StVincent;
		impl Sealed for StVincent {
			const NAME: &'static str = "America/St_Vincent";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for StVincent {}
		/// The `America/Swift_Current` time zone.
		pub struct SwiftCurrent;
		impl Sealed for SwiftCurrent {
			const NAME: &'static str = "America/Swift_Current";
			const DATA: TzifData = AMERICA_SWIFT_CURRENT;
		}
		impl TimeZone for SwiftCurrent {}
		/// The `America/Tegucigalpa` time zone.
		pub struct Tegucigalpa;
		impl Sealed for Tegucigalpa {
			const NAME: &'static str = "America/Tegucigalpa";
			const DATA: TzifData = AMERICA_TEGUCIGALPA;
		}
		impl TimeZone for Tegucigalpa {}
		/// The `America/Thule` time zone.
		pub struct Thule;
		impl Sealed for Thule {
			const NAME: &'static str = "America/Thule";
			const DATA: TzifData = AMERICA_THULE;
		}
		impl TimeZone for Thule {}
		/// The `America/Thunder_Bay` time zone.
		pub struct ThunderBay;
		impl Sealed for ThunderBay {
			const NAME: &'static str = "America/Thunder_Bay";
			const DATA: TzifData = CANADA_EASTERN;
		}
		impl TimeZone for ThunderBay {}
		/// The `America/Tijuana` time zone.
		pub struct Tijuana;
		impl Sealed for Tijuana {
			const NAME: &'static str = "America/Tijuana";
			const DATA: TzifData = MEXICO_BAJA_NORTE;
		}
		impl TimeZone for Tijuana {}
		/// The `America/Toronto` time zone.
		pub struct Toronto;
		impl Sealed for Toronto {
			const NAME: &'static str = "America/Toronto";
			const DATA: TzifData = CANADA_EASTERN;
		}
		impl TimeZone for Toronto {}
		/// The `America/Tortola` time zone.
		pub struct Tortola;
		impl Sealed for Tortola {
			const NAME: &'static str = "America/Tortola";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Tortola {}
		/// The `America/Vancouver` time zone.
		pub struct Vancouver;
		impl Sealed for Vancouver {
			const NAME: &'static str = "America/Vancouver";
			const DATA: TzifData = CANADA_PACIFIC;
		}
		impl TimeZone for Vancouver {}
		/// The `America/Virgin` time zone.
		pub struct Virgin;
		impl Sealed for Virgin {
			const NAME: &'static str = "America/Virgin";
			const DATA: TzifData = AMERICA_VIRGIN;
		}
		impl TimeZone for Virgin {}
		/// The `America/Whitehorse` time zone.
		pub struct Whitehorse;
		impl Sealed for Whitehorse {
			const NAME: &'static str = "America/Whitehorse";
			const DATA: TzifData = CANADA_YUKON;
		}
		impl TimeZone for Whitehorse {}
		/// The `America/Winnipeg` time zone.
		pub struct Winnipeg;
		impl Sealed for Winnipeg {
			const NAME: &'static str = "America/Winnipeg";
			const DATA: TzifData = CANADA_CENTRAL;
		}
		impl TimeZone for Winnipeg {}
		/// The `America/Yakutat` time zone.
		pub struct Yakutat;
		impl Sealed for Yakutat {
			const NAME: &'static str = "America/Yakutat";
			const DATA: TzifData = AMERICA_YAKUTAT;
		}
		impl TimeZone for Yakutat {}
		/// The `America/Yellowknife` time zone.
		pub struct Yellowknife;
		impl Sealed for Yellowknife {
			const NAME: &'static str = "America/Yellowknife";
			const DATA: TzifData = CANADA_MOUNTAIN;
		}
		impl TimeZone for Yellowknife {}
	}
	pub mod Antarctica {
		use super::*;
		/// The `Antarctica/Casey` time zone.
		pub struct Casey;
		impl Sealed for Casey {
			const NAME: &'static str = "Antarctica/Casey";
			const DATA: TzifData = ANTARCTICA_CASEY;
		}
		impl TimeZone for Casey {}
		/// The `Antarctica/Davis` time zone.
		pub struct Davis;
		impl Sealed for Davis {
			const NAME: &'static str = "Antarctica/Davis";
			const DATA: TzifData = ANTARCTICA_DAVIS;
		}
		impl TimeZone for Davis {}
		/// The `Antarctica/DumontDUrville` time zone.
		pub struct DumontDUrville;
		impl Sealed for DumontDUrville {
			const NAME: &'static str = "Antarctica/DumontDUrville";
			const DATA: TzifData = PACIFIC_YAP;
		}
		impl TimeZone for DumontDUrville {}
		/// The `Antarctica/Macquarie` time zone.
		pub struct Macquarie;
		impl Sealed for Macquarie {
			const NAME: &'static str = "Antarctica/Macquarie";
			const DATA: TzifData = ANTARCTICA_MACQUARIE;
		}
		impl TimeZone for Macquarie {}
		/// The `Antarctica/Mawson` time zone.
		pub struct Mawson;
		impl Sealed for Mawson {
			const NAME: &'static str = "Antarctica/Mawson";
			const DATA: TzifData = ANTARCTICA_MAWSON;
		}
		impl TimeZone for Mawson {}
		/// The `Antarctica/McMurdo` time zone.
		pub struct McMurdo;
		impl Sealed for McMurdo {
			const NAME: &'static str = "Antarctica/McMurdo";
			const DATA: TzifData = NZ;
		}
		impl TimeZone for McMurdo {}
		/// The `Antarctica/Palmer` time zone.
		pub struct Palmer;
		impl Sealed for Palmer {
			const NAME: &'static str = "Antarctica/Palmer";
			const DATA: TzifData = ANTARCTICA_PALMER;
		}
		impl TimeZone for Palmer {}
		/// The `Antarctica/Rothera` time zone.
		pub struct Rothera;
		impl Sealed for Rothera {
			const NAME: &'static str = "Antarctica/Rothera";
			const DATA: TzifData = ANTARCTICA_ROTHERA;
		}
		impl TimeZone for Rothera {}
		/// The `Antarctica/South_Pole` time zone.
		pub struct SouthPole;
		impl Sealed for SouthPole {
			const NAME: &'static str = "Antarctica/South_Pole";
			const DATA: TzifData = NZ;
		}
		impl TimeZone for SouthPole {}
		/// The `Antarctica/Syowa` time zone.
		pub struct Syowa;
		impl Sealed for Syowa {
			const NAME: &'static str = "Antarctica/Syowa";
			const DATA: TzifData = ASIA_KUWAIT;
		}
		impl TimeZone for Syowa {}
		/// The `Antarctica/Troll` time zone.
		pub struct Troll;
		impl Sealed for Troll {
			const NAME: &'static str = "Antarctica/Troll";
			const DATA: TzifData = ANTARCTICA_TROLL;
		}
		impl TimeZone for Troll {}
		/// The `Antarctica/Vostok` time zone.
		pub struct Vostok;
		impl Sealed for Vostok {
			const NAME: &'static str = "Antarctica/Vostok";
			const DATA: TzifData = ASIA_KASHGAR;
		}
		impl TimeZone for Vostok {}
	}
	pub mod Arctic {
		use super::*;
		/// The `Arctic/Longyearbyen` time zone.
		pub struct Longyearbyen;
		impl Sealed for Longyearbyen {
			const NAME: &'static str = "Arctic/Longyearbyen";
			const DATA: TzifData = ARCTIC_LONGYEARBYEN;
		}
		impl TimeZone for Longyearbyen {}
	}
	pub mod Asia {
		use super::*;
		/// The `Asia/Aden` time zone.
		pub struct Aden;
		impl Sealed for Aden {
			const NAME: &'static str = "Asia/Aden";
			const DATA: TzifData = ASIA_KUWAIT;
		}
		impl TimeZone for Aden {}
		/// The `Asia/Almaty` time zone.
		pub struct Almaty;
		impl Sealed for Almaty {
			const NAME: &'static str = "Asia/Almaty";
			const DATA: TzifData = ASIA_ALMATY;
		}
		impl TimeZone for Almaty {}
		/// The `Asia/Amman` time zone.
		pub struct Amman;
		impl Sealed for Amman {
			const NAME: &'static str = "Asia/Amman";
			const DATA: TzifData = ASIA_AMMAN;
		}
		impl TimeZone for Amman {}
		/// The `Asia/Anadyr` time zone.
		pub struct Anadyr;
		impl Sealed for Anadyr {
			const NAME: &'static str = "Asia/Anadyr";
			const DATA: TzifData = ASIA_ANADYR;
		}
		impl TimeZone for Anadyr {}
		/// The `Asia/Aqtau` time zone.
		pub struct Aqtau;
		impl Sealed for Aqtau {
			const NAME: &'static str = "Asia/Aqtau";
			const DATA: TzifData = ASIA_AQTAU;
		}
		impl TimeZone for Aqtau {}
		/// The `Asia/Aqtobe` time zone.
		pub struct Aqtobe;
		impl Sealed for Aqtobe {
			const NAME: &'static str = "Asia/Aqtobe";
			const DATA: TzifData = ASIA_AQTOBE;
		}
		impl TimeZone for Aqtobe {}
		/// The `Asia/Ashgabat` time zone.
		pub struct Ashgabat;
		impl Sealed for Ashgabat {
			const NAME: &'static str = "Asia/Ashgabat";
			const DATA: TzifData = ASIA_ASHKHABAD;
		}
		impl TimeZone for Ashgabat {}
		/// The `Asia/Ashkhabad` time zone.
		pub struct Ashkhabad;
		impl Sealed for Ashkhabad {
			const NAME: &'static str = "Asia/Ashkhabad";
			const DATA: TzifData = ASIA_ASHKHABAD;
		}
		impl TimeZone for Ashkhabad {}
		/// The `Asia/Atyrau` time zone.
		pub struct Atyrau;
		impl Sealed for Atyrau {
			const NAME: &'static str = "Asia/Atyrau";
			const DATA: TzifData = ASIA_ATYRAU;
		}
		impl TimeZone for Atyrau {}
		/// The `Asia/Baghdad` time zone.
		pub struct Baghdad;
		impl Sealed for Baghdad {
			const NAME: &'static str = "Asia/Baghdad";
			const DATA: TzifData = ASIA_BAGHDAD;
		}
		impl TimeZone for Baghdad {}
		/// The `Asia/Bahrain` time zone.
		pub struct Bahrain;
		impl Sealed for Bahrain {
			const NAME: &'static str = "Asia/Bahrain";
			const DATA: TzifData = ASIA_BAHRAIN;
		}
		impl TimeZone for Bahrain {}
		/// The `Asia/Baku` time zone.
		pub struct Baku;
		impl Sealed for Baku {
			const NAME: &'static str = "Asia/Baku";
			const DATA: TzifData = ASIA_BAKU;
		}
		impl TimeZone for Baku {}
		/// The `Asia/Bangkok` time zone.
		pub struct Bangkok;
		impl Sealed for Bangkok {
			const NAME: &'static str = "Asia/Bangkok";
			const DATA: TzifData = ASIA_VIENTIANE;
		}
		impl TimeZone for Bangkok {}
		/// The `Asia/Barnaul` time zone.
		pub struct Barnaul;
		impl Sealed for Barnaul {
			const NAME: &'static str = "Asia/Barnaul";
			const DATA: TzifData = ASIA_BARNAUL;
		}
		impl TimeZone for Barnaul {}
		/// The `Asia/Beirut` time zone.
		pub struct Beirut;
		impl Sealed for Beirut {
			const NAME: &'static str = "Asia/Beirut";
			const DATA: TzifData = ASIA_BEIRUT;
		}
		impl TimeZone for Beirut {}
		/// The `Asia/Bishkek` time zone.
		pub struct Bishkek;
		impl Sealed for Bishkek {
			const NAME: &'static str = "Asia/Bishkek";
			const DATA: TzifData = ASIA_BISHKEK;
		}
		impl TimeZone for Bishkek {}
		/// The `Asia/Brunei` time zone.
		pub struct Brunei;
		impl Sealed for Brunei {
			const NAME: &'static str = "Asia/Brunei";
			const DATA: TzifData = ASIA_BRUNEI;
		}
		impl TimeZone for Brunei {}
		/// The `Asia/Calcutta` time zone.
		pub struct Calcutta;
		impl Sealed for Calcutta {
			const NAME: &'static str = "Asia/Calcutta";
			const DATA: TzifData = ASIA_CALCUTTA;
		}
		impl TimeZone for Calcutta {}
		/// The `Asia/Chita` time zone.
		pub struct Chita;
		impl Sealed for Chita {
			const NAME: &'static str = "Asia/Chita";
			const DATA: TzifData = ASIA_CHITA;
		}
		impl TimeZone for Chita {}
		/// The `Asia/Choibalsan` time zone.
		pub struct Choibalsan;
		impl Sealed for Choibalsan {
			const NAME: &'static str = "Asia/Choibalsan";
			const DATA: TzifData = ASIA_CHOIBALSAN;
		}
		impl TimeZone for Choibalsan {}
		/// The `Asia/Chongqing` time zone.
		pub struct Chongqing;
		impl Sealed for Chongqing {
			const NAME: &'static str = "Asia/Chongqing";
			const DATA: TzifData = PRC;
		}
		impl TimeZone for Chongqing {}
		/// The `Asia/Chungking` time zone.
		pub struct Chungking;
		impl Sealed for Chungking {
			const NAME: &'static str = "Asia/Chungking";
			const DATA: TzifData = PRC;
		}
		impl TimeZone for Chungking {}
		/// The `Asia/Colombo` time zone.
		pub struct Colombo;
		impl Sealed for Colombo {
			const NAME: &'static str = "Asia/Colombo";
			const DATA: TzifData = ASIA_COLOMBO;
		}
		impl TimeZone for Colombo {}
		/// The `Asia/Dacca` time zone.
		pub struct Dacca;
		impl Sealed for Dacca {
			const NAME: &'static str = "Asia/Dacca";
			const DATA: TzifData = ASIA_DACCA;
		}
		impl TimeZone for Dacca {}
		/// The `Asia/Damascus` time zone.
		pub struct Damascus;
		impl Sealed for Damascus {
			const NAME: &'static str = "Asia/Damascus";
			const DATA: TzifData = ASIA_DAMASCUS;
		}
		impl TimeZone for Damascus {}
		/// The `Asia/Dhaka` time zone.
		pub struct Dhaka;
		impl Sealed for Dhaka {
			const NAME: &'static str = "Asia/Dhaka";
			const DATA: TzifData = ASIA_DACCA;
		}
		impl TimeZone for Dhaka {}
		/// The `Asia/Dili` time zone.
		pub struct Dili;
		impl Sealed for Dili {
			const NAME: &'static str = "Asia/Dili";
			const DATA: TzifData = ASIA_DILI;
		}
		impl TimeZone for Dili {}
		/// The `Asia/Dubai` time zone.
		pub struct Dubai;
		impl Sealed for Dubai {
			const NAME: &'static str = "Asia/Dubai";
			const DATA: TzifData = ASIA_MUSCAT;
		}
		impl TimeZone for Dubai {}
		/// The `Asia/Dushanbe` time zone.
		pub struct Dushanbe;
		impl Sealed for Dushanbe {
			const NAME: &'static str = "Asia/Dushanbe";
			const DATA: TzifData = ASIA_DUSHANBE;
		}
		impl TimeZone for Dushanbe {}
		/// The `Asia/Famagusta` time zone.
		pub struct Famagusta;
		impl Sealed for Famagusta {
			const NAME: &'static str = "Asia/Famagusta";
			const DATA: TzifData = ASIA_FAMAGUSTA;
		}
		impl TimeZone for Famagusta {}
		/// The `Asia/Gaza` time zone.
		pub struct Gaza;
		impl Sealed for Gaza {
			const NAME: &'static str = "Asia/Gaza";
			const DATA: TzifData = ASIA_GAZA;
		}
		impl TimeZone for Gaza {}
		/// The `Asia/Harbin` time zone.
		pub struct Harbin;
		impl Sealed for Harbin {
			const NAME: &'static str = "Asia/Harbin";
			const DATA: TzifData = PRC;
		}
		impl TimeZone for Harbin {}
		/// The `Asia/Hebron` time zone.
		pub struct Hebron;
		impl Sealed for Hebron {
			const NAME: &'static str = "Asia/Hebron";
			const DATA: TzifData = ASIA_HEBRON;
		}
		impl TimeZone for Hebron {}
		/// The `Asia/Ho_Chi_Minh` time zone.
		pub struct HoChiMinh;
		impl Sealed for HoChiMinh {
			const NAME: &'static str = "Asia/Ho_Chi_Minh";
			const DATA: TzifData = ASIA_SAIGON;
		}
		impl TimeZone for HoChiMinh {}
		/// The `Asia/Hong_Kong` time zone.
		pub struct HongKong;
		impl Sealed for HongKong {
			const NAME: &'static str = "Asia/Hong_Kong";
			const DATA: TzifData = HONGKONG;
		}
		impl TimeZone for HongKong {}
		/// The `Asia/Hovd` time zone.
		pub struct Hovd;
		impl Sealed for Hovd {
			const NAME: &'static str = "Asia/Hovd";
			const DATA: TzifData = ASIA_HOVD;
		}
		impl TimeZone for Hovd {}
		/// The `Asia/Irkutsk` time zone.
		pub struct Irkutsk;
		impl Sealed for Irkutsk {
			const NAME: &'static str = "Asia/Irkutsk";
			const DATA: TzifData = ASIA_IRKUTSK;
		}
		impl TimeZone for Irkutsk {}
		/// The `Asia/Istanbul` time zone.
		pub struct Istanbul;
		impl Sealed for Istanbul {
			const NAME: &'static str = "Asia/Istanbul";
			const DATA: TzifData = TURKEY;
		}
		impl TimeZone for Istanbul {}
		/// The `Asia/Jakarta` time zone.
		pub struct Jakarta;
		impl Sealed for Jakarta {
			const NAME: &'static str = "Asia/Jakarta";
			const DATA: TzifData = ASIA_JAKARTA;
		}
		impl TimeZone for Jakarta {}
		/// The `Asia/Jayapura` time zone.
		pub struct Jayapura;
		impl Sealed for Jayapura {
			const NAME: &'static str = "Asia/Jayapura";
			const DATA: TzifData = ASIA_JAYAPURA;
		}
		impl TimeZone for Jayapura {}
		/// The `Asia/Jerusalem` time zone.
		pub struct Jerusalem;
		impl Sealed for Jerusalem {
			const NAME: &'static str = "Asia/Jerusalem";
			const DATA: TzifData = ISRAEL;
		}
		impl TimeZone for Jerusalem {}
		/// The `Asia/Kabul` time zone.
		pub struct Kabul;
		impl Sealed for Kabul {
			const NAME: &'static str = "Asia/Kabul";
			const DATA: TzifData = ASIA_KABUL;
		}
		impl TimeZone for Kabul {}
		/// The `Asia/Kamchatka` time zone.
		pub struct Kamchatka;
		impl Sealed for Kamchatka {
			const NAME: &'static str = "Asia/Kamchatka";
			const DATA: TzifData = ASIA_KAMCHATKA;
		}
		impl TimeZone for Kamchatka {}
		/// The `Asia/Karachi` time zone.
		pub struct Karachi;
		impl Sealed for Karachi {
			const NAME: &'static str = "Asia/Karachi";
			const DATA: TzifData = ASIA_KARACHI;
		}
		impl TimeZone for Karachi {}
		/// The `Asia/Kashgar` time zone.
		pub struct Kashgar;
		impl Sealed for Kashgar {
			const NAME: &'static str = "Asia/Kashgar";
			const DATA: TzifData = ASIA_KASHGAR;
		}
		impl TimeZone for Kashgar {}
		/// The `Asia/Kathmandu` time zone.
		pub struct Kathmandu;
		impl Sealed for Kathmandu {
			const NAME: &'static str = "Asia/Kathmandu";
			const DATA: TzifData = ASIA_KATMANDU;
		}
		impl TimeZone for Kathmandu {}
		/// The `Asia/Katmandu` time zone.
		pub struct Katmandu;
		impl Sealed for Katmandu {
			const NAME: &'static str = "Asia/Katmandu";
			const DATA: TzifData = ASIA_KATMANDU;
		}
		impl TimeZone for Katmandu {}
		/// The `Asia/Khandyga` time zone.
		pub struct Khandyga;
		impl Sealed for Khandyga {
			const NAME: &'static str = "Asia/Khandyga";
			const DATA: TzifData = ASIA_KHANDYGA;
		}
		impl TimeZone for Khandyga {}
		/// The `Asia/Kolkata` time zone.
		pub struct Kolkata;
		impl Sealed for Kolkata {
			const NAME: &'static str = "Asia/Kolkata";
			const DATA: TzifData = ASIA_CALCUTTA;
		}
		impl TimeZone for Kolkata {}
		/// The `Asia/Krasnoyarsk` time zone.
		pub struct Krasnoyarsk;
		impl Sealed for Krasnoyarsk {
			const NAME: &'static str = "Asia/Krasnoyarsk";
			const DATA: TzifData = ASIA_KRASNOYARSK;
		}
		impl TimeZone for Krasnoyarsk {}
		/// The `Asia/Kuala_Lumpur` time zone.
		pub struct KualaLumpur;
		impl Sealed for KualaLumpur {
			const NAME: &'static str = "Asia/Kuala_Lumpur";
			const DATA: TzifData = SINGAPORE;
		}
		impl TimeZone for KualaLumpur {}
		/// The `Asia/Kuching` time zone.
		pub struct Kuching;
		impl Sealed for Kuching {
			const NAME: &'static str = "Asia/Kuching";
			const DATA: TzifData = ASIA_BRUNEI;
		}
		impl TimeZone for Kuching {}
		/// The `Asia/Kuwait` time zone.
		pub struct Kuwait;
		impl Sealed for Kuwait {
			const NAME: &'static str = "Asia/Kuwait";
			const DATA: TzifData = ASIA_KUWAIT;
		}
		impl TimeZone for Kuwait {}
		/// The `Asia/Macao` time zone.
		pub struct Macao;
		impl Sealed for Macao {
			const NAME: &'static str = "Asia/Macao";
			const DATA: TzifData = ASIA_MACAO;
		}
		impl TimeZone for Macao {}
		/// The `Asia/Macau` time zone.
		pub struct Macau;
		impl Sealed for Macau {
			const NAME: &'static str = "Asia/Macau";
			const DATA: TzifData = ASIA_MACAO;
		}
		impl TimeZone for Macau {}
		/// The `Asia/Magadan` time zone.
		pub struct Magadan;
		impl Sealed for Magadan {
			const NAME: &'static str = "Asia/Magadan";
			const DATA: TzifData = ASIA_MAGADAN;
		}
		impl TimeZone for Magadan {}
		/// The `Asia/Makassar` time zone.
		pub struct Makassar;
		impl Sealed for Makassar {
			const NAME: &'static str = "Asia/Makassar";
			const DATA: TzifData = ASIA_UJUNG_PANDANG;
		}
		impl TimeZone for Makassar {}
		/// The `Asia/Manila` time zone.
		pub struct Manila;
		impl Sealed for Manila {
			const NAME: &'static str = "Asia/Manila";
			const DATA: TzifData = ASIA_MANILA;
		}
		impl TimeZone for Manila {}
		/// The `Asia/Muscat` time zone.
		pub struct Muscat;
		impl Sealed for Muscat {
			const NAME: &'static str = "Asia/Muscat";
			const DATA: TzifData = ASIA_MUSCAT;
		}
		impl TimeZone for Muscat {}
		/// The `Asia/Nicosia` time zone.
		pub struct Nicosia;
		impl Sealed for Nicosia {
			const NAME: &'static str = "Asia/Nicosia";
			const DATA: TzifData = EUROPE_NICOSIA;
		}
		impl TimeZone for Nicosia {}
		/// The `Asia/Novokuznetsk` time zone.
		pub struct Novokuznetsk;
		impl Sealed for Novokuznetsk {
			const NAME: &'static str = "Asia/Novokuznetsk";
			const DATA: TzifData = ASIA_NOVOKUZNETSK;
		}
		impl TimeZone for Novokuznetsk {}
		/// The `Asia/Novosibirsk` time zone.
		pub struct Novosibirsk;
		impl Sealed for Novosibirsk {
			const NAME: &'static str = "Asia/Novosibirsk";
			const DATA: TzifData = ASIA_NOVOSIBIRSK;
		}
		impl TimeZone for Novosibirsk {}
		/// The `Asia/Omsk` time zone.
		pub struct Omsk;
		impl Sealed for Omsk {
			const NAME: &'static str = "Asia/Omsk";
			const DATA: TzifData = ASIA_OMSK;
		}
		impl TimeZone for Omsk {}
		/// The `Asia/Oral` time zone.
		pub struct Oral;
		impl Sealed for Oral {
			const NAME: &'static str = "Asia/Oral";
			const DATA: TzifData = ASIA_ORAL;
		}
		impl TimeZone for Oral {}
		/// The `Asia/Phnom_Penh` time zone.
		pub struct PhnomPenh;
		impl Sealed for PhnomPenh {
			const NAME: &'static str = "Asia/Phnom_Penh";
			const DATA: TzifData = ASIA_VIENTIANE;
		}
		impl TimeZone for PhnomPenh {}
		/// The `Asia/Pontianak` time zone.
		pub struct Pontianak;
		impl Sealed for Pontianak {
			const NAME: &'static str = "Asia/Pontianak";
			const DATA: TzifData = ASIA_PONTIANAK;
		}
		impl TimeZone for Pontianak {}
		/// The `Asia/Pyongyang` time zone.
		pub struct Pyongyang;
		impl Sealed for Pyongyang {
			const NAME: &'static str = "Asia/Pyongyang";
			const DATA: TzifData = ASIA_PYONGYANG;
		}
		impl TimeZone for Pyongyang {}
		/// The `Asia/Qatar` time zone.
		pub struct Qatar;
		impl Sealed for Qatar {
			const NAME: &'static str = "Asia/Qatar";
			const DATA: TzifData = ASIA_BAHRAIN;
		}
		impl TimeZone for Qatar {}
		/// The `Asia/Qostanay` time zone.
		pub struct Qostanay;
		impl Sealed for Qostanay {
			const NAME: &'static str = "Asia/Qostanay";
			const DATA: TzifData = ASIA_QOSTANAY;
		}
		impl TimeZone for Qostanay {}
		/// The `Asia/Qyzylorda` time zone.
		pub struct Qyzylorda;
		impl Sealed for Qyzylorda {
			const NAME: &'static str = "Asia/Qyzylorda";
			const DATA: TzifData = ASIA_QYZYLORDA;
		}
		impl TimeZone for Qyzylorda {}
		/// The `Asia/Rangoon` time zone.
		pub struct Rangoon;
		impl Sealed for Rangoon {
			const NAME: &'static str = "Asia/Rangoon";
			const DATA: TzifData = ASIA_RANGOON;
		}
		impl TimeZone for Rangoon {}
		/// The `Asia/Riyadh` time zone.
		pub struct Riyadh;
		impl Sealed for Riyadh {
			const NAME: &'static str = "Asia/Riyadh";
			const DATA: TzifData = ASIA_KUWAIT;
		}
		impl TimeZone for Riyadh {}
		/// The `Asia/Saigon` time zone.
		pub struct Saigon;
		impl Sealed for Saigon {
			const NAME: &'static str = "Asia/Saigon";
			const DATA: TzifData = ASIA_SAIGON;
		}
		impl TimeZone for Saigon {}
		/// The `Asia/Sakhalin` time zone.
		pub struct Sakhalin;
		impl Sealed for Sakhalin {
			const NAME: &'static str = "Asia/Sakhalin";
			const DATA: TzifData = ASIA_SAKHALIN;
		}
		impl TimeZone for Sakhalin {}
		/// The `Asia/Samarkand` time zone.
		pub struct Samarkand;
		impl Sealed for Samarkand {
			const NAME: &'static str = "Asia/Samarkand";
			const DATA: TzifData = ASIA_SAMARKAND;
		}
		impl TimeZone for Samarkand {}
		/// The `Asia/Seoul` time zone.
		pub struct Seoul;
		impl Sealed for Seoul {
			const NAME: &'static str = "Asia/Seoul";
			const DATA: TzifData = ROK;
		}
		impl TimeZone for Seoul {}
		/// The `Asia/Shanghai` time zone.
		pub struct Shanghai;
		impl Sealed for Shanghai {
			const NAME: &'static str = "Asia/Shanghai";
			const DATA: TzifData = PRC;
		}
		impl TimeZone for Shanghai {}
		/// The `Asia/Singapore` time zone.
		pub struct Singapore;
		impl Sealed for Singapore {
			const NAME: &'static str = "Asia/Singapore";
			const DATA: TzifData = SINGAPORE;
		}
		impl TimeZone for Singapore {}
		/// The `Asia/Srednekolymsk` time zone.
		pub struct Srednekolymsk;
		impl Sealed for Srednekolymsk {
			const NAME: &'static str = "Asia/Srednekolymsk";
			const DATA: TzifData = ASIA_SREDNEKOLYMSK;
		}
		impl TimeZone for Srednekolymsk {}
		/// The `Asia/Taipei` time zone.
		pub struct Taipei;
		impl Sealed for Taipei {
			const NAME: &'static str = "Asia/Taipei";
			const DATA: TzifData = ROC;
		}
		impl TimeZone for Taipei {}
		/// The `Asia/Tashkent` time zone.
		pub struct Tashkent;
		impl Sealed for Tashkent {
			const NAME: &'static str = "Asia/Tashkent";
			const DATA: TzifData = ASIA_TASHKENT;
		}
		impl TimeZone for Tashkent {}
		/// The `Asia/Tbilisi` time zone.
		pub struct Tbilisi;
		impl Sealed for Tbilisi {
			const NAME: &'static str = "Asia/Tbilisi";
			const DATA: TzifData = ASIA_TBILISI;
		}
		impl TimeZone for Tbilisi {}
		/// The `Asia/Tehran` time zone.
		pub struct Tehran;
		impl Sealed for Tehran {
			const NAME: &'static str = "Asia/Tehran";
			const DATA: TzifData = IRAN;
		}
		impl TimeZone for Tehran {}
		/// The `Asia/Tel_Aviv` time zone.
		pub struct TelAviv;
		impl Sealed for TelAviv {
			const NAME: &'static str = "Asia/Tel_Aviv";
			const DATA: TzifData = ISRAEL;
		}
		impl TimeZone for TelAviv {}
		/// The `Asia/Thimbu` time zone.
		pub struct Thimbu;
		impl Sealed for Thimbu {
			const NAME: &'static str = "Asia/Thimbu";
			const DATA: TzifData = ASIA_THIMBU;
		}
		impl TimeZone for Thimbu {}
		/// The `Asia/Thimphu` time zone.
		pub struct Thimphu;
		impl Sealed for Thimphu {
			const NAME: &'static str = "Asia/Thimphu";
			const DATA: TzifData = ASIA_THIMBU;
		}
		impl TimeZone for Thimphu {}
		/// The `Asia/Tokyo` time zone.
		pub struct Tokyo;
		impl Sealed for Tokyo {
			const NAME: &'static str = "Asia/Tokyo";
			const DATA: TzifData = JAPAN;
		}
		impl TimeZone for Tokyo {}
		/// The `Asia/Tomsk` time zone.
		pub struct Tomsk;
		impl Sealed for Tomsk {
			const NAME: &'static str = "Asia/Tomsk";
			const DATA: TzifData = ASIA_TOMSK;
		}
		impl TimeZone for Tomsk {}
		/// The `Asia/Ujung_Pandang` time zone.
		pub struct UjungPandang;
		impl Sealed for UjungPandang {
			const NAME: &'static str = "Asia/Ujung_Pandang";
			const DATA: TzifData = ASIA_UJUNG_PANDANG;
		}
		impl TimeZone for UjungPandang {}
		/// The `Asia/Ulaanbaatar` time zone.
		pub struct Ulaanbaatar;
		impl Sealed for Ulaanbaatar {
			const NAME: &'static str = "Asia/Ulaanbaatar";
			const DATA: TzifData = ASIA_ULAN_BATOR;
		}
		impl TimeZone for Ulaanbaatar {}
		/// The `Asia/Ulan_Bator` time zone.
		pub struct UlanBator;
		impl Sealed for UlanBator {
			const NAME: &'static str = "Asia/Ulan_Bator";
			const DATA: TzifData = ASIA_ULAN_BATOR;
		}
		impl TimeZone for UlanBator {}
		/// The `Asia/Urumqi` time zone.
		pub struct Urumqi;
		impl Sealed for Urumqi {
			const NAME: &'static str = "Asia/Urumqi";
			const DATA: TzifData = ASIA_KASHGAR;
		}
		impl TimeZone for Urumqi {}
		/// The `Asia/Ust-Nera` time zone.
		pub struct UstNera;
		impl Sealed for UstNera {
			const NAME: &'static str = "Asia/Ust-Nera";
			const DATA: TzifData = ASIA_UST_NERA;
		}
		impl TimeZone for UstNera {}
		/// The `Asia/Vientiane` time zone.
		pub struct Vientiane;
		impl Sealed for Vientiane {
			const NAME: &'static str = "Asia/Vientiane";
			const DATA: TzifData = ASIA_VIENTIANE;
		}
		impl TimeZone for Vientiane {}
		/// The `Asia/Vladivostok` time zone.
		pub struct Vladivostok;
		impl Sealed for Vladivostok {
			const NAME: &'static str = "Asia/Vladivostok";
			const DATA: TzifData = ASIA_VLADIVOSTOK;
		}
		impl TimeZone for Vladivostok {}
		/// The `Asia/Yakutsk` time zone.
		pub struct Yakutsk;
		impl Sealed for Yakutsk {
			const NAME: &'static str = "Asia/Yakutsk";
			const DATA: TzifData = ASIA_YAKUTSK;
		}
		impl TimeZone for Yakutsk {}
		/// The `Asia/Yangon` time zone.
		pub struct Yangon;
		impl Sealed for Yangon {
			const NAME: &'static str = "Asia/Yangon";
			const DATA: TzifData = ASIA_RANGOON;
		}
		impl TimeZone for Yangon {}
		/// The `Asia/Yekaterinburg` time zone.
		pub struct Yekaterinburg;
		impl Sealed for Yekaterinburg {
			const NAME: &'static str = "Asia/Yekaterinburg";
			const DATA: TzifData = ASIA_YEKATERINBURG;
		}
		impl TimeZone for Yekaterinburg {}
		/// The `Asia/Yerevan` time zone.
		pub struct Yerevan;
		impl Sealed for Yerevan {
			const NAME: &'static str = "Asia/Yerevan";
			const DATA: TzifData = ASIA_YEREVAN;
		}
		impl TimeZone for Yerevan {}
	}
	pub mod Atlantic {
		use super::*;
		/// The `Atlantic/Azores` time zone.
		pub struct Azores;
		impl Sealed for Azores {
			const NAME: &'static str = "Atlantic/Azores";
			const DATA: TzifData = ATLANTIC_AZORES;
		}
		impl TimeZone for Azores {}
		/// The `Atlantic/Bermuda` time zone.
		pub struct Bermuda;
		impl Sealed for Bermuda {
			const NAME: &'static str = "Atlantic/Bermuda";
			const DATA: TzifData = ATLANTIC_BERMUDA;
		}
		impl TimeZone for Bermuda {}
		/// The `Atlantic/Canary` time zone.
		pub struct Canary;
		impl Sealed for Canary {
			const NAME: &'static str = "Atlantic/Canary";
			const DATA: TzifData = ATLANTIC_CANARY;
		}
		impl TimeZone for Canary {}
		/// The `Atlantic/Cape_Verde` time zone.
		pub struct CapeVerde;
		impl Sealed for CapeVerde {
			const NAME: &'static str = "Atlantic/Cape_Verde";
			const DATA: TzifData = ATLANTIC_CAPE_VERDE;
		}
		impl TimeZone for CapeVerde {}
		/// The `Atlantic/Faeroe` time zone.
		pub struct Faeroe;
		impl Sealed for Faeroe {
			const NAME: &'static str = "Atlantic/Faeroe";
			const DATA: TzifData = ATLANTIC_FAEROE;
		}
		impl TimeZone for Faeroe {}
		/// The `Atlantic/Faroe` time zone.
		pub struct Faroe;
		impl Sealed for Faroe {
			const NAME: &'static str = "Atlantic/Faroe";
			const DATA: TzifData = ATLANTIC_FAEROE;
		}
		impl TimeZone for Faroe {}
		/// The `Atlantic/Jan_Mayen` time zone.
		pub struct JanMayen;
		impl Sealed for JanMayen {
			const NAME: &'static str = "Atlantic/Jan_Mayen";
			const DATA: TzifData = ARCTIC_LONGYEARBYEN;
		}
		impl TimeZone for JanMayen {}
		/// The `Atlantic/Madeira` time zone.
		pub struct Madeira;
		impl Sealed for Madeira {
			const NAME: &'static str = "Atlantic/Madeira";
			const DATA: TzifData = ATLANTIC_MADEIRA;
		}
		impl TimeZone for Madeira {}
		/// The `Atlantic/Reykjavik` time zone.
		pub struct Reykjavik;
		impl Sealed for Reykjavik {
			const NAME: &'static str = "Atlantic/Reykjavik";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for Reykjavik {}
		/// The `Atlantic/South_Georgia` time zone.
		pub struct SouthGeorgia;
		impl Sealed for SouthGeorgia {
			const NAME: &'static str = "Atlantic/South_Georgia";
			const DATA: TzifData = ATLANTIC_SOUTH_GEORGIA;
		}
		impl TimeZone for SouthGeorgia {}
		/// The `Atlantic/St_Helena` time zone.
		pub struct StHelena;
		impl Sealed for StHelena {
			const NAME: &'static str = "Atlantic/St_Helena";
			const DATA: TzifData = ICELAND;
		}
		impl TimeZone for StHelena {}
		/// The `Atlantic/Stanley` time zone.
		pub struct Stanley;
		impl Sealed for Stanley {
			const NAME: &'static str = "Atlantic/Stanley";
			const DATA: TzifData = ATLANTIC_STANLEY;
		}
		impl TimeZone for Stanley {}
	}
	pub mod Australia {
		use super::*;
		/// The `Australia/ACT` time zone.
		pub struct Act;
		impl Sealed for Act {
			const NAME: &'static str = "Australia/ACT";
			const DATA: TzifData = AUSTRALIA_NSW;
		}
		impl TimeZone for Act {}
		/// The `Australia/Adelaide` time zone.
		pub struct Adelaide;
		impl Sealed for Adelaide {
			const NAME: &'static str = "Australia/Adelaide";
			const DATA: TzifData = AUSTRALIA_SOUTH;
		}
		impl TimeZone for Adelaide {}
		/// The `Australia/Brisbane` time zone.
		pub struct Brisbane;
		impl Sealed for Brisbane {
			const NAME: &'static str = "Australia/Brisbane";
			const DATA: TzifData = AUSTRALIA_QUEENSLAND;
		}
		impl TimeZone for Brisbane {}
		/// The `Australia/Broken_Hill` time zone.
		pub struct BrokenHill;
		impl Sealed for BrokenHill {
			const NAME: &'static str = "Australia/Broken_Hill";
			const DATA: TzifData = AUSTRALIA_YANCOWINNA;
		}
		impl TimeZone for BrokenHill {}
		/// The `Australia/Canberra` time zone.
		pub struct Canberra;
		impl Sealed for Canberra {
			const NAME: &'static str = "Australia/Canberra";
			const DATA: TzifData = AUSTRALIA_NSW;
		}
		impl TimeZone for Canberra {}
		/// The `Australia/Currie` time zone.
		pub struct Currie;
		impl Sealed for Currie {
			const NAME: &'static str = "Australia/Currie";
			const DATA: TzifData = AUSTRALIA_TASMANIA;
		}
		impl TimeZone for Currie {}
		/// The `Australia/Darwin` time zone.
		pub struct Darwin;
		impl Sealed for Darwin {
			const NAME: &'static str = "Australia/Darwin";
			const DATA: TzifData = AUSTRALIA_NORTH;
		}
		impl TimeZone for Darwin {}
		/// The `Australia/Eucla` time zone.
		pub struct Eucla;
		impl Sealed for Eucla {
			const NAME: &'static str = "Australia/Eucla";
			const DATA: TzifData = AUSTRALIA_EUCLA;
		}
		impl TimeZone for Eucla {}
		/// The `Australia/Hobart` time zone.
		pub struct Hobart;
		impl Sealed for Hobart {
			const NAME: &'static str = "Australia/Hobart";
			const DATA: TzifData = AUSTRALIA_TASMANIA;
		}
		impl TimeZone for Hobart {}
		/// The `Australia/LHI` time zone.
		pub struct Lhi;
		impl Sealed for Lhi {
			const NAME: &'static str = "Australia/LHI";
			const DATA: TzifData = AUSTRALIA_LHI;
		}
		impl TimeZone for Lhi {}
		/// The `Australia/Lindeman` time zone.
		pub struct Lindeman;
		impl Sealed for Lindeman {
			const NAME: &'static str = "Australia/Lindeman";
			const DATA: TzifData = AUSTRALIA_LINDEMAN;
		}
		impl TimeZone for Lindeman {}
		/// The `Australia/Lord_Howe` time zone.
		pub struct LordHowe;
		impl Sealed for LordHowe {
			const NAME: &'static str = "Australia/Lord_Howe";
			const DATA: TzifData = AUSTRALIA_LHI;
		}
		impl TimeZone for LordHowe {}
		/// The `Australia/Melbourne` time zone.
		pub struct Melbourne;
		impl Sealed for Melbourne {
			const NAME: &'static str = "Australia/Melbourne";
			const DATA: TzifData = AUSTRALIA_VICTORIA;
		}
		impl TimeZone for Melbourne {}
		/// The `Australia/NSW` time zone.
		pub struct Nsw;
		impl Sealed for Nsw {
			const NAME: &'static str = "Australia/NSW";
			const DATA: TzifData = AUSTRALIA_NSW;
		}
		impl TimeZone for Nsw {}
		/// The `Australia/North` time zone.
		pub struct North;
		impl Sealed for North {
			const NAME: &'static str = "Australia/North";
			const DATA: TzifData = AUSTRALIA_NORTH;
		}
		impl TimeZone for North {}
		/// The `Australia/Perth` time zone.
		pub struct Perth;
		impl Sealed for Perth {
			const NAME: &'static str = "Australia/Perth";
			const DATA: TzifData = AUSTRALIA_WEST;
		}
		impl TimeZone for Perth {}
		/// The `Australia/Queensland` time zone.
		pub struct Queensland;
		impl Sealed for Queensland {
			const NAME: &'static str = "Australia/Queensland";
			const DATA: TzifData = AUSTRALIA_QUEENSLAND;
		}
		impl TimeZone for Queensland {}
		/// The `Australia/South` time zone.
		pub struct South;
		impl Sealed for South {
			const NAME: &'static str = "Australia/South";
			const DATA: TzifData = AUSTRALIA_SOUTH;
		}
		impl TimeZone for South {}
		/// The `Australia/Sydney` time zone.
		pub struct Sydney;
		impl Sealed for Sydney {
			const NAME: &'static str = "Australia/Sydney";
			const DATA: TzifData = AUSTRALIA_NSW;
		}
		impl TimeZone for Sydney {}
		/// The `Australia/Tasmania` time zone.
		pub struct Tasmania;
		impl Sealed for Tasmania {
			const NAME: &'static str = "Australia/Tasmania";
			const DATA: TzifData = AUSTRALIA_TASMANIA;
		}
		impl TimeZone for Tasmania {}
		/// The `Australia/Victoria` time zone.
		pub struct Victoria;
		impl Sealed for Victoria {
			const NAME: &'static str = "Australia/Victoria";
			const DATA: TzifData = AUSTRALIA_VICTORIA;
		}
		impl TimeZone for Victoria {}
		/// The `Australia/West` time zone.
		pub struct West;
		impl Sealed for West {
			const NAME: &'static str = "Australia/West";
			const DATA: TzifData = AUSTRALIA_WEST;
		}
		impl TimeZone for West {}
		/// The `Australia/Yancowinna` time zone.
		pub struct Yancowinna;
		impl Sealed for Yancowinna {
			const NAME: &'static str = "Australia/Yancowinna";
			const DATA: TzifData = AUSTRALIA_YANCOWINNA;
		}
		impl TimeZone for Yancowinna {}
	}
	pub mod Brazil {
		use super::*;
		/// The `Brazil/Acre` time zone.
		pub struct Acre;
		impl Sealed for Acre {
			const NAME: &'static str = "Brazil/Acre";
			const DATA: TzifData = BRAZIL_ACRE;
		}
		impl TimeZone for Acre {}
		/// The `Brazil/DeNoronha` time zone.
		pub struct DeNoronha;
		impl Sealed for DeNoronha {
			const NAME: &'static str = "Brazil/DeNoronha";
			const DATA: TzifData = BRAZIL_DE_NORONHA;
		}
		impl TimeZone for DeNoronha {}
		/// The `Brazil/East` time zone.
		pub struct East;
		impl Sealed for East {
			const NAME: &'static str = "Brazil/East";
			const DATA: TzifData = BRAZIL_EAST;
		}
		impl TimeZone for East {}
		/// The `Brazil/West` time zone.
		pub struct West;
		impl Sealed for West {
			const NAME: &'static str = "Brazil/West";
			const DATA: TzifData = BRAZIL_WEST;
		}
		impl TimeZone for West {}
	}
	/// The `CET` time zone.
	pub struct Cet;
	impl Sealed for Cet {
		const NAME: &'static str = "CET";
		const DATA: TzifData = CET;
	}
	impl TimeZone for Cet {}
	/// The `CST6CDT` time zone.
	pub struct Cst6Cdt;
	impl Sealed for Cst6Cdt {
		const NAME: &'static str = "CST6CDT";
		const DATA: TzifData = CST_6_CDT;
	}
	impl TimeZone for Cst6Cdt {}
	pub mod Canada {
		use super::*;
		/// The `Canada/Atlantic` time zone.
		pub struct Atlantic;
		impl Sealed for Atlantic {
			const NAME: &'static str = "Canada/Atlantic";
			const DATA: TzifData = CANADA_ATLANTIC;
		}
		impl TimeZone for Atlantic {}
		/// The `Canada/Central` time zone.
		pub struct Central;
		impl Sealed for Central {
			const NAME: &'static str = "Canada/Central";
			const DATA: TzifData = CANADA_CENTRAL;
		}
		impl TimeZone for Central {}
		/// The `Canada/Eastern` time zone.
		pub struct Eastern;
		impl Sealed for Eastern {
			const NAME: &'static str = "Canada/Eastern";
			const DATA: TzifData = CANADA_EASTERN;
		}
		impl TimeZone for Eastern {}
		/// The `Canada/Mountain` time zone.
		pub struct Mountain;
		impl Sealed for Mountain {
			const NAME: &'static str = "Canada/Mountain";
			const DATA: TzifData = CANADA_MOUNTAIN;
		}
		impl TimeZone for Mountain {}
		/// The `Canada/Newfoundland` time zone.
		pub struct Newfoundland;
		impl Sealed for Newfoundland {
			const NAME: &'static str = "Canada/Newfoundland";
			const DATA: TzifData = CANADA_NEWFOUNDLAND;
		}
		impl TimeZone for Newfoundland {}
		/// The `Canada/Pacific` time zone.
		pub struct Pacific;
		impl Sealed for Pacific {
			const NAME: &'static str = "Canada/Pacific";
			const DATA: TzifData = CANADA_PACIFIC;
		}
		impl TimeZone for Pacific {}
		/// The `Canada/Saskatchewan` time zone.
		pub struct Saskatchewan;
		impl Sealed for Saskatchewan {
			const NAME: &'static str = "Canada/Saskatchewan";
			const DATA: TzifData = CANADA_SASKATCHEWAN;
		}
		impl TimeZone for Saskatchewan {}
		/// The `Canada/Yukon` time zone.
		pub struct Yukon;
		impl Sealed for Yukon {
			const NAME: &'static str = "Canada/Yukon";
			const DATA: TzifData = CANADA_YUKON;
		}
		impl TimeZone for Yukon {}
	}
	pub mod Chile {
		use super::*;
		/// The `Chile/Continental` time zone.
		pub struct Continental;
		impl Sealed for Continental {
			const NAME: &'static str = "Chile/Continental";
			const DATA: TzifData = CHILE_CONTINENTAL;
		}
		impl TimeZone for Continental {}
		/// The `Chile/EasterIsland` time zone.
		pub struct EasterIsland;
		impl Sealed for EasterIsland {
			const NAME: &'static str = "Chile/EasterIsland";
			const DATA: TzifData = CHILE_EASTER_ISLAND;
		}
		impl TimeZone for EasterIsland {}
	}
	/// The `Cuba` time zone.
	pub struct Cuba;
	impl Sealed for Cuba {
		const NAME: &'static str = "Cuba";
		const DATA: TzifData = CUBA;
	}
	impl TimeZone for Cuba {}
	/// The `EET` time zone.
	pub struct Eet;
	impl Sealed for Eet {
		const NAME: &'static str = "EET";
		const DATA: TzifData = EET;
	}
	impl TimeZone for Eet {}
	/// The `EST` time zone.
	pub struct Est;
	impl Sealed for Est {
		const NAME: &'static str = "EST";
		const DATA: TzifData = EST;
	}
	impl TimeZone for Est {}
	/// The `EST5EDT` time zone.
	pub struct Est5Edt;
	impl Sealed for Est5Edt {
		const NAME: &'static str = "EST5EDT";
		const DATA: TzifData = EST_5_EDT;
	}
	impl TimeZone for Est5Edt {}
	/// The `Egypt` time zone.
	pub struct Egypt;
	impl Sealed for Egypt {
		const NAME: &'static str = "Egypt";
		const DATA: TzifData = EGYPT;
	}
	impl TimeZone for Egypt {}
	/// The `Eire` time zone.
	pub struct Eire;
	impl Sealed for Eire {
		const NAME: &'static str = "Eire";
		const DATA: TzifData = EIRE;
	}
	impl TimeZone for Eire {}
	pub mod Etc {
		use super::*;
		/// The `Etc/GMT` time zone.
		pub struct Gmt;
		impl Sealed for Gmt {
			const NAME: &'static str = "Etc/GMT";
			const DATA: TzifData = GREENWICH;
		}
		impl TimeZone for Gmt {}
		/// The `Etc/GMT+0` time zone.
		pub struct GmtPlus0;
		impl Sealed for GmtPlus0 {
			const NAME: &'static str = "Etc/GMT+0";
			const DATA: TzifData = GREENWICH;
		}
		impl TimeZone for GmtPlus0 {}
		/// The `Etc/GMT+1` time zone.
		pub struct GmtPlus1;
		impl Sealed for GmtPlus1 {
			const NAME: &'static str = "Etc/GMT+1";
			const DATA: TzifData = ETC_GMT_PLUS_1;
		}
		impl TimeZone for GmtPlus1 {}
		/// The `Etc/GMT+10` time zone.
		pub struct GmtPlus10;
		impl Sealed for GmtPlus10 {
			const NAME: &'static str = "Etc/GMT+10";
			const DATA: TzifData = ETC_GMT_PLUS_10;
		}
		impl TimeZone for GmtPlus10 {}
		/// The `Etc/GMT+11` time zone.
		pub struct GmtPlus11;
		impl Sealed for GmtPlus11 {
			const NAME: &'static str = "Etc/GMT+11";
			const DATA: TzifData = ETC_GMT_PLUS_11;
		}
		impl TimeZone for GmtPlus11 {}
		/// The `Etc/GMT+12` time zone.
		pub struct GmtPlus12;
		impl Sealed for GmtPlus12 {
			const NAME: &'static str = "Etc/GMT+12";
			const DATA: TzifData = ETC_GMT_PLUS_12;
		}
		impl TimeZone for GmtPlus12 {}
		/// The `Etc/GMT+2` time zone.
		pub struct GmtPlus2;
		impl Sealed for GmtPlus2 {
			const NAME: &'static str = "Etc/GMT+2";
			const DATA: TzifData = ETC_GMT_PLUS_2;
		}
		impl TimeZone for GmtPlus2 {}
		/// The `Etc/GMT+3` time zone.
		pub struct GmtPlus3;
		impl Sealed for GmtPlus3 {
			const NAME: &'static str = "Etc/GMT+3";
			const DATA: TzifData = ETC_GMT_PLUS_3;
		}
		impl TimeZone for GmtPlus3 {}
		/// The `Etc/GMT+4` time zone.
		pub struct GmtPlus4;
		impl Sealed for GmtPlus4 {
			const NAME: &'static str = "Etc/GMT+4";
			const DATA: TzifData = ETC_GMT_PLUS_4;
		}
		impl TimeZone for GmtPlus4 {}
		/// The `Etc/GMT+5` time zone.
		pub struct GmtPlus5;
		impl Sealed for GmtPlus5 {
			const NAME: &'static str = "Etc/GMT+5";
			const DATA: TzifData = ETC_GMT_PLUS_5;
		}
		impl TimeZone for GmtPlus5 {}
		/// The `Etc/GMT+6` time zone.
		pub struct GmtPlus6;
		impl Sealed for GmtPlus6 {
			const NAME: &'static str = "Etc/GMT+6";
			const DATA: TzifData = ETC_GMT_PLUS_6;
		}
		impl TimeZone for GmtPlus6 {}
		/// The `Etc/GMT+7` time zone.
		pub struct GmtPlus7;
		impl Sealed for GmtPlus7 {
			const NAME: &'static str = "Etc/GMT+7";
			const DATA: TzifData = ETC_GMT_PLUS_7;
		}
		impl TimeZone for GmtPlus7 {}
		/// The `Etc/GMT+8` time zone.
		pub struct GmtPlus8;
		impl Sealed for GmtPlus8 {
			const NAME: &'static str = "Etc/GMT+8";
			const DATA: TzifData = ETC_GMT_PLUS_8;
		}
		impl TimeZone for GmtPlus8 {}
		/// The `Etc/GMT+9` time zone.
		pub struct GmtPlus9;
		impl Sealed for GmtPlus9 {
			const NAME: &'static str = "Etc/GMT+9";
			const DATA: TzifData = ETC_GMT_PLUS_9;
		}
		impl TimeZone for GmtPlus9 {}
		/// The `Etc/GMT-0` time zone.
		pub struct GmtMinus0;
		impl Sealed for GmtMinus0 {
			const NAME: &'static str = "Etc/GMT-0";
			const DATA: TzifData = GREENWICH;
		}
		impl TimeZone for GmtMinus0 {}
		/// The `Etc/GMT-1` time zone.
		pub struct GmtMinus1;
		impl Sealed for GmtMinus1 {
			const NAME: &'static str = "Etc/GMT-1";
			const DATA: TzifData = ETC_GMT_MINUS_1;
		}
		impl TimeZone for GmtMinus1 {}
		/// The `Etc/GMT-10` time zone.
		pub struct GmtMinus10;
		impl Sealed for GmtMinus10 {
			const NAME: &'static str = "Etc/GMT-10";
			const DATA: TzifData = ETC_GMT_MINUS_10;
		}
		impl TimeZone for GmtMinus10 {}
		/// The `Etc/GMT-11` time zone.
		pub struct GmtMinus11;
		impl Sealed for GmtMinus11 {
			const NAME: &'static str = "Etc/GMT-11";
			const DATA: TzifData = ETC_GMT_MINUS_11;
		}
		impl TimeZone for GmtMinus11 {}
		/// The `Etc/GMT-12` time zone.
		pub struct GmtMinus12;
		impl Sealed for GmtMinus12 {
			const NAME: &'static str = "Etc/GMT-12";
			const DATA: TzifData = ETC_GMT_MINUS_12;
		}
		impl TimeZone for GmtMinus12 {}
		/// The `Etc/GMT-13` time zone.
		pub struct GmtMinus13;
		impl Sealed for GmtMinus13 {
			const NAME: &'static str = "Etc/GMT-13";
			const DATA: TzifData = ETC_GMT_MINUS_13;
		}
		impl TimeZone for GmtMinus13 {}
		/// The `Etc/GMT-14` time zone.
		pub struct GmtMinus14;
		impl Sealed for GmtMinus14 {
			const NAME: &'static str = "Etc/GMT-14";
			const DATA: TzifData = ETC_GMT_MINUS_14;
		}
		impl TimeZone for GmtMinus14 {}
		/// The `Etc/GMT-2` time zone.
		pub struct GmtMinus2;
		impl Sealed for GmtMinus2 {
			const NAME: &'static str = "Etc/GMT-2";
			const DATA: TzifData = ETC_GMT_MINUS_2;
		}
		impl TimeZone for GmtMinus2 {}
		/// The `Etc/GMT-3` time zone.
		pub struct GmtMinus3;
		impl Sealed for GmtMinus3 {
			const NAME: &'static str = "Etc/GMT-3";
			const DATA: TzifData = ETC_GMT_MINUS_3;
		}
		impl TimeZone for GmtMinus3 {}
		/// The `Etc/GMT-4` time zone.
		pub struct GmtMinus4;
		impl Sealed for GmtMinus4 {
			const NAME: &'static str = "Etc/GMT-4";
			const DATA: TzifData = ETC_GMT_MINUS_4;
		}
		impl TimeZone for GmtMinus4 {}
		/// The `Etc/GMT-5` time zone.
		pub struct GmtMinus5;
		impl Sealed for GmtMinus5 {
			const NAME: &'static str = "Etc/GMT-5";
			const DATA: TzifData = ETC_GMT_MINUS_5;
		}
		impl TimeZone for GmtMinus5 {}
		/// The `Etc/GMT-6` time zone.
		pub struct GmtMinus6;
		impl Sealed for GmtMinus6 {
			const NAME: &'static str = "Etc/GMT-6";
			const DATA: TzifData = ETC_GMT_MINUS_6;
		}
		impl TimeZone for GmtMinus6 {}
		/// The `Etc/GMT-7` time zone.
		pub struct GmtMinus7;
		impl Sealed for GmtMinus7 {
			const NAME: &'static str = "Etc/GMT-7";
			const DATA: TzifData = ETC_GMT_MINUS_7;
		}
		impl TimeZone for GmtMinus7 {}
		/// The `Etc/GMT-8` time zone.
		pub struct GmtMinus8;
		impl Sealed for GmtMinus8 {
			const NAME: &'static str = "Etc/GMT-8";
			const DATA: TzifData = ETC_GMT_MINUS_8;
		}
		impl TimeZone for GmtMinus8 {}
		/// The `Etc/GMT-9` time zone.
		pub struct GmtMinus9;
		impl Sealed for GmtMinus9 {
			const NAME: &'static str = "Etc/GMT-9";
			const DATA: TzifData = ETC_GMT_MINUS_9;
		}
		impl TimeZone for GmtMinus9 {}
		/// The `Etc/GMT0` time zone.
		pub struct Gmt0;
		impl Sealed for Gmt0 {
			const NAME: &'static str = "Etc/GMT0";
			const DATA: TzifData = GREENWICH;
		}
		impl TimeZone for Gmt0 {}
		/// The `Etc/Greenwich` time zone.
		pub struct Greenwich;
		impl Sealed for Greenwich {
			const NAME: &'static str = "Etc/Greenwich";
			const DATA: TzifData = GREENWICH;
		}
		impl TimeZone for Greenwich {}
		/// The `Etc/UCT` time zone.
		pub struct Uct;
		impl Sealed for Uct {
			const NAME: &'static str = "Etc/UCT";
			const DATA: TzifData = ZULU;
		}
		impl TimeZone for Uct {}
		/// The `Etc/UTC` time zone.
		pub struct Utc;
		impl Sealed for Utc {
			const NAME: &'static str = "Etc/UTC";
			const DATA: TzifData = ZULU;
		}
		impl TimeZone for Utc {}
		/// The `Etc/Universal` time zone.
		pub struct Universal;
		impl Sealed for Universal {
			const NAME: &'static str = "Etc/Universal";
			const DATA: TzifData = ZULU;
		}
		impl TimeZone for Universal {}
		/// The `Etc/Zulu` time zone.
		pub struct Zulu;
		impl Sealed for Zulu {
			const NAME: &'static str = "Etc/Zulu";
			const DATA: TzifData = ZULU;
		}
		impl TimeZone for Zulu {}
	}
	pub mod Europe {
		use super::*;
		/// The `Europe/Amsterdam` time zone.
		pub struct Amsterdam;
		impl Sealed for Amsterdam {
			const NAME: &'static str = "Europe/Amsterdam";
			const DATA: TzifData = EUROPE_LUXEMBOURG;
		}
		impl TimeZone for Amsterdam {}
		/// The `Europe/Andorra` time zone.
		pub struct Andorra;
		impl Sealed for Andorra {
			const NAME: &'static str = "Europe/Andorra";
			const DATA: TzifData = EUROPE_ANDORRA;
		}
		impl TimeZone for Andorra {}
		/// The `Europe/Astrakhan` time zone.
		pub struct Astrakhan;
		impl Sealed for Astrakhan {
			const NAME: &'static str = "Europe/Astrakhan";
			const DATA: TzifData = EUROPE_ASTRAKHAN;
		}
		impl TimeZone for Astrakhan {}
		/// The `Europe/Athens` time zone.
		pub struct Athens;
		impl Sealed for Athens {
			const NAME: &'static str = "Europe/Athens";
			const DATA: TzifData = EUROPE_ATHENS;
		}
		impl TimeZone for Athens {}
		/// The `Europe/Belfast` time zone.
		pub struct Belfast;
		impl Sealed for Belfast {
			const NAME: &'static str = "Europe/Belfast";
			const DATA: TzifData = GB_EIRE;
		}
		impl TimeZone for Belfast {}
		/// The `Europe/Belgrade` time zone.
		pub struct Belgrade;
		impl Sealed for Belgrade {
			const NAME: &'static str = "Europe/Belgrade";
			const DATA: TzifData = EUROPE_ZAGREB;
		}
		impl TimeZone for Belgrade {}
		/// The `Europe/Berlin` time zone.
		pub struct Berlin;
		impl Sealed for Berlin {
			const NAME: &'static str = "Europe/Berlin";
			const DATA: TzifData = ARCTIC_LONGYEARBYEN;
		}
		impl TimeZone for Berlin {}
		/// The `Europe/Bratislava` time zone.
		pub struct Bratislava;
		impl Sealed for Bratislava {
			const NAME: &'static str = "Europe/Bratislava";
			const DATA: TzifData = EUROPE_BRATISLAVA;
		}
		impl TimeZone for Bratislava {}
		/// The `Europe/Brussels` time zone.
		pub struct Brussels;
		impl Sealed for Brussels {
			const NAME: &'static str = "Europe/Brussels";
			const DATA: TzifData = EUROPE_LUXEMBOURG;
		}
		impl TimeZone for Brussels {}
		/// The `Europe/Bucharest` time zone.
		pub struct Bucharest;
		impl Sealed for Bucharest {
			const NAME: &'static str = "Europe/Bucharest";
			const DATA: TzifData = EUROPE_BUCHAREST;
		}
		impl TimeZone for Bucharest {}
		/// The `Europe/Budapest` time zone.
		pub struct Budapest;
		impl Sealed for Budapest {
			const NAME: &'static str = "Europe/Budapest";
			const DATA: TzifData = EUROPE_BUDAPEST;
		}
		impl TimeZone for Budapest {}
		/// The `Europe/Busingen` time zone.
		pub struct Busingen;
		impl Sealed for Busingen {
			const NAME: &'static str = "Europe/Busingen";
			const DATA: TzifData = EUROPE_VADUZ;
		}
		impl TimeZone for Busingen {}
		/// The `Europe/Chisinau` time zone.
		pub struct Chisinau;
		impl Sealed for Chisinau {
			const NAME: &'static str = "Europe/Chisinau";
			const DATA: TzifData = EUROPE_TIRASPOL;
		}
		impl TimeZone for Chisinau {}
		/// The `Europe/Copenhagen` time zone.
		pub struct Copenhagen;
		impl Sealed for Copenhagen {
			const NAME: &'static str = "Europe/Copenhagen";
			const DATA: TzifData = ARCTIC_LONGYEARBYEN;
		}
		impl TimeZone for Copenhagen {}
		/// The `Europe/Dublin` time zone.
		pub struct Dublin;
		impl Sealed for Dublin {
			const NAME: &'static str = "Europe/Dublin";
			const DATA: TzifData = EIRE;
		}
		impl TimeZone for Dublin {}
		/// The `Europe/Gibraltar` time zone.
		pub struct Gibraltar;
		impl Sealed for Gibraltar {
			const NAME: &'static str = "Europe/Gibraltar";
			const DATA: TzifData = EUROPE_GIBRALTAR;
		}
		impl TimeZone for Gibraltar {}
		/// The `Europe/Guernsey` time zone.
		pub struct Guernsey;
		impl Sealed for Guernsey {
			const NAME: &'static str = "Europe/Guernsey";
			const DATA: TzifData = GB_EIRE;
		}
		impl TimeZone for Guernsey {}
		/// The `Europe/Helsinki` time zone.
		pub struct Helsinki;
		impl Sealed for Helsinki {
			const NAME: &'static str = "Europe/Helsinki";
			const DATA: TzifData = EUROPE_MARIEHAMN;
		}
		impl TimeZone for Helsinki {}
		/// The `Europe/Isle_of_Man` time zone.
		pub struct IsleOfMan;
		impl Sealed for IsleOfMan {
			const NAME: &'static str = "Europe/Isle_of_Man";
			const DATA: TzifData = GB_EIRE;
		}
		impl TimeZone for IsleOfMan {}
		/// The `Europe/Istanbul` time zone.
		pub struct Istanbul;
		impl Sealed for Istanbul {
			const NAME: &'static str = "Europe/Istanbul";
			const DATA: TzifData = TURKEY;
		}
		impl TimeZone for Istanbul {}
		/// The `Europe/Jersey` time zone.
		pub struct Jersey;
		impl Sealed for Jersey {
			const NAME: &'static str = "Europe/Jersey";
			const DATA: TzifData = GB_EIRE;
		}
		impl TimeZone for Jersey {}
		/// The `Europe/Kaliningrad` time zone.
		pub struct Kaliningrad;
		impl Sealed for Kaliningrad {
			const NAME: &'static str = "Europe/Kaliningrad";
			const DATA: TzifData = EUROPE_KALININGRAD;
		}
		impl TimeZone for Kaliningrad {}
		/// The `Europe/Kiev` time zone.
		pub struct Kiev;
		impl Sealed for Kiev {
			const NAME: &'static str = "Europe/Kiev";
			const DATA: TzifData = EUROPE_ZAPOROZHYE;
		}
		impl TimeZone for Kiev {}
		/// The `Europe/Kirov` time zone.
		pub struct Kirov;
		impl Sealed for Kirov {
			const NAME: &'static str = "Europe/Kirov";
			const DATA: TzifData = EUROPE_KIROV;
		}
		impl TimeZone for Kirov {}
		/// The `Europe/Kyiv` time zone.
		pub struct Kyiv;
		impl Sealed for Kyiv {
			const NAME: &'static str = "Europe/Kyiv";
			const DATA: TzifData = EUROPE_ZAPOROZHYE;
		}
		impl TimeZone for Kyiv {}
		/// The `Europe/Lisbon` time zone.
		pub struct Lisbon;
		impl Sealed for Lisbon {
			const NAME: &'static str = "Europe/Lisbon";
			const DATA: TzifData = PORTUGAL;
		}
		impl TimeZone for Lisbon {}
		/// The `Europe/Ljubljana` time zone.
		pub struct Ljubljana;
		impl Sealed for Ljubljana {
			const NAME: &'static str = "Europe/Ljubljana";
			const DATA: TzifData = EUROPE_ZAGREB;
		}
		impl TimeZone for Ljubljana {}
		/// The `Europe/London` time zone.
		pub struct London;
		impl Sealed for London {
			const NAME: &'static str = "Europe/London";
			const DATA: TzifData = GB_EIRE;
		}
		impl TimeZone for London {}
		/// The `Europe/Luxembourg` time zone.
		pub struct Luxembourg;
		impl Sealed for Luxembourg {
			const NAME: &'static str = "Europe/Luxembourg";
			const DATA: TzifData = EUROPE_LUXEMBOURG;
		}
		impl TimeZone for Luxembourg {}
		/// The `Europe/Madrid` time zone.
		pub struct Madrid;
		impl Sealed for Madrid {
			const NAME: &'static str = "Europe/Madrid";
			const DATA: TzifData = EUROPE_MADRID;
		}
		impl TimeZone for Madrid {}
		/// The `Europe/Malta` time zone.
		pub struct Malta;
		impl Sealed for Malta {
			const NAME: &'static str = "Europe/Malta";
			const DATA: TzifData = EUROPE_MALTA;
		}
		impl TimeZone for Malta {}
		/// The `Europe/Mariehamn` time zone.
		pub struct Mariehamn;
		impl Sealed for Mariehamn {
			const NAME: &'static str = "Europe/Mariehamn";
			const DATA: TzifData = EUROPE_MARIEHAMN;
		}
		impl TimeZone for Mariehamn {}
		/// The `Europe/Minsk` time zone.
		pub struct Minsk;
		impl Sealed for Minsk {
			const NAME: &'static str = "Europe/Minsk";
			const DATA: TzifData = EUROPE_MINSK;
		}
		impl TimeZone for Minsk {}
		/// The `Europe/Monaco` time zone.
		pub struct Monaco;
		impl Sealed for Monaco {
			const NAME: &'static str = "Europe/Monaco";
			const DATA: TzifData = EUROPE_MONACO;
		}
		impl TimeZone for Monaco {}
		/// The `Europe/Moscow` time zone.
		pub struct Moscow;
		impl Sealed for Moscow {
			const NAME: &'static str = "Europe/Moscow";
			const DATA: TzifData = W_SU;
		}
		impl TimeZone for Moscow {}
		/// The `Europe/Nicosia` time zone.
		pub struct Nicosia;
		impl Sealed for Nicosia {
			const NAME: &'static str = "Europe/Nicosia";
			const DATA: TzifData = EUROPE_NICOSIA;
		}
		impl TimeZone for Nicosia {}
		/// The `Europe/Oslo` time zone.
		pub struct Oslo;
		impl Sealed for Oslo {
			const NAME: &'static str = "Europe/Oslo";
			const DATA: TzifData = ARCTIC_LONGYEARBYEN;
		}
		impl TimeZone for Oslo {}
		/// The `Europe/Paris` time zone.
		pub struct Paris;
		impl Sealed for Paris {
			const NAME: &'static str = "Europe/Paris";
			const DATA: TzifData = EUROPE_MONACO;
		}
		impl TimeZone for Paris {}
		/// The `Europe/Podgorica` time zone.
		pub struct Podgorica;
		impl Sealed for Podgorica {
			const NAME: &'static str = "Europe/Podgorica";
			const DATA: TzifData = EUROPE_ZAGREB;
		}
		impl TimeZone for Podgorica {}
		/// The `Europe/Prague` time zone.
		pub struct Prague;
		impl Sealed for Prague {
			const NAME: &'static str = "Europe/Prague";
			const DATA: TzifData = EUROPE_BRATISLAVA;
		}
		impl TimeZone for Prague {}
		/// The `Europe/Riga` time zone.
		pub struct Riga;
		impl Sealed for Riga {
			const NAME: &'static str = "Europe/Riga";
			const DATA: TzifData = EUROPE_RIGA;
		}
		impl TimeZone for Riga {}
		/// The `Europe/Rome` time zone.
		pub struct Rome;
		impl Sealed for Rome {
			const NAME: &'static str = "Europe/Rome";
			const DATA: TzifData = EUROPE_VATICAN;
		}
		impl TimeZone for Rome {}
		/// The `Europe/Samara` time zone.
		pub struct Samara;
		impl Sealed for Samara {
			const NAME: &'static str = "Europe/Samara";
			const DATA: TzifData = EUROPE_SAMARA;
		}
		impl TimeZone for Samara {}
		/// The `Europe/San_Marino` time zone.
		pub struct SanMarino;
		impl Sealed for SanMarino {
			const NAME: &'static str = "Europe/San_Marino";
			const DATA: TzifData = EUROPE_VATICAN;
		}
		impl TimeZone for SanMarino {}
		/// The `Europe/Sarajevo` time zone.
		pub struct Sarajevo;
		impl Sealed for Sarajevo {
			const NAME: &'static str = "Europe/Sarajevo";
			const DATA: TzifData = EUROPE_ZAGREB;
		}
		impl TimeZone for Sarajevo {}
		/// The `Europe/Saratov` time zone.
		pub struct Saratov;
		impl Sealed for Saratov {
			const NAME: &'static str = "Europe/Saratov";
			const DATA: TzifData = EUROPE_SARATOV;
		}
		impl TimeZone for Saratov {}
		/// The `Europe/Simferopol` time zone.
		pub struct Simferopol;
		impl Sealed for Simferopol {
			const NAME: &'static str = "Europe/Simferopol";
			const DATA: TzifData = EUROPE_SIMFEROPOL;
		}
		impl TimeZone for Simferopol {}
		/// The `Europe/Skopje` time zone.
		pub struct Skopje;
		impl Sealed for Skopje {
			const NAME: &'static str = "Europe/Skopje";
			const DATA: TzifData = EUROPE_ZAGREB;
		}
		impl TimeZone for Skopje {}
		/// The `Europe/Sofia` time zone.
		pub struct Sofia;
		impl Sealed for Sofia {
			const NAME: &'static str = "Europe/Sofia";
			const DATA: TzifData = EUROPE_SOFIA;
		}
		impl TimeZone for Sofia {}
		/// The `Europe/Stockholm` time zone.
		pub struct Stockholm;
		impl Sealed for Stockholm {
			const NAME: &'static str = "Europe/Stockholm";
			const DATA: TzifData = ARCTIC_LONGYEARBYEN;
		}
		impl TimeZone for Stockholm {}
		/// The `Europe/Tallinn` time zone.
		pub struct Tallinn;
		impl Sealed for Tallinn {
			const NAME: &'static str = "Europe/Tallinn";
			const DATA: TzifData = EUROPE_TALLINN;
		}
		impl TimeZone for Tallinn {}
		/// The `Europe/Tirane` time zone.
		pub struct Tirane;
		impl Sealed for Tirane {
			const NAME: &'static str = "Europe/Tirane";
			const DATA: TzifData = EUROPE_TIRANE;
		}
		impl TimeZone for Tirane {}
		/// The `Europe/Tiraspol` time zone.
		pub struct Tiraspol;
		impl Sealed for Tiraspol {
			const NAME: &'static str = "Europe/Tiraspol";
			const DATA: TzifData = EUROPE_TIRASPOL;
		}
		impl TimeZone for Tiraspol {}
		/// The `Europe/Ulyanovsk` time zone.
		pub struct Ulyanovsk;
		impl Sealed for Ulyanovsk {
			const NAME: &'static str = "Europe/Ulyanovsk";
			const DATA: TzifData = EUROPE_ULYANOVSK;
		}
		impl TimeZone for Ulyanovsk {}
		/// The `Europe/Uzhgorod` time zone.
		pub struct Uzhgorod;
		impl Sealed for Uzhgorod {
			const NAME: &'static str = "Europe/Uzhgorod";
			const DATA: TzifData = EUROPE_ZAPOROZHYE;
		}
		impl TimeZone for Uzhgorod {}
		/// The `Europe/Vaduz` time zone.
		pub struct Vaduz;
		impl Sealed for Vaduz {
			const NAME: &'static str = "Europe/Vaduz";
			const DATA: TzifData = EUROPE_VADUZ;
		}
		impl TimeZone for Vaduz {}
		/// The `Europe/Vatican` time zone.
		pub struct Vatican;
		impl Sealed for Vatican {
			const NAME: &'static str = "Europe/Vatican";
			const DATA: TzifData = EUROPE_VATICAN;
		}
		impl TimeZone for Vatican {}
		/// The `Europe/Vienna` time zone.
		pub struct Vienna;
		impl Sealed for Vienna {
			const NAME: &'static str = "Europe/Vienna";
			const DATA: TzifData = EUROPE_VIENNA;
		}
		impl TimeZone for Vienna {}
		/// The `Europe/Vilnius` time zone.
		pub struct Vilnius;
		impl Sealed for Vilnius {
			const NAME: &'static str = "Europe/Vilnius";
			const DATA: TzifData = EUROPE_VILNIUS;
		}
		impl TimeZone for Vilnius {}
		/// The `Europe/Volgograd` time zone.
		pub struct Volgograd;
		impl Sealed for Volgograd {
			const NAME: &'static str = "Europe/Volgograd";
			const DATA: TzifData = EUROPE_VOLGOGRAD;
		}
		impl TimeZone for Volgograd {}
		/// The `Europe/Warsaw` time zone.
		pub struct Warsaw;
		impl Sealed for Warsaw {
			const NAME: &'static str = "Europe/Warsaw";
			const DATA: TzifData = POLAND;
		}
		impl TimeZone for Warsaw {}
		/// The `Europe/Zagreb` time zone.
		pub struct Zagreb;
		impl Sealed for Zagreb {
			const NAME: &'static str = "Europe/Zagreb";
			const DATA: TzifData = EUROPE_ZAGREB;
		}
		impl TimeZone for Zagreb {}
		/// The `Europe/Zaporozhye` time zone.
		pub struct Zaporozhye;
		impl Sealed for Zaporozhye {
			const NAME: &'static str = "Europe/Zaporozhye";
			const DATA: TzifData = EUROPE_ZAPOROZHYE;
		}
		impl TimeZone for Zaporozhye {}
		/// The `Europe/Zurich` time zone.
		pub struct Zurich;
		impl Sealed for Zurich {
			const NAME: &'static str = "Europe/Zurich";
			const DATA: TzifData = EUROPE_VADUZ;
		}
		impl TimeZone for Zurich {}
	}
	/// The `Factory` time zone.
	pub struct Factory;
	impl Sealed for Factory {
		const NAME: &'static str = "Factory";
		const DATA: TzifData = FACTORY;
	}
	impl TimeZone for Factory {}
	/// The `GB` time zone.
	pub struct Gb;
	impl Sealed for Gb {
		const NAME: &'static str = "GB";
		const DATA: TzifData = GB_EIRE;
	}
	impl TimeZone for Gb {}
	/// The `GB-Eire` time zone.
	pub struct GbEire;
	impl Sealed for GbEire {
		const NAME: &'static str = "GB-Eire";
		const DATA: TzifData = GB_EIRE;
	}
	impl TimeZone for GbEire {}
	/// The `GMT` time zone.
	pub struct Gmt;
	impl Sealed for Gmt {
		const NAME: &'static str = "GMT";
		const DATA: TzifData = GREENWICH;
	}
	impl TimeZone for Gmt {}
	/// The `GMT+0` time zone.
	pub struct GmtPlus0;
	impl Sealed for GmtPlus0 {
		const NAME: &'static str = "GMT+0";
		const DATA: TzifData = GREENWICH;
	}
	impl TimeZone for GmtPlus0 {}
	/// The `GMT-0` time zone.
	pub struct GmtMinus0;
	impl Sealed for GmtMinus0 {
		const NAME: &'static str = "GMT-0";
		const DATA: TzifData = GREENWICH;
	}
	impl TimeZone for GmtMinus0 {}
	/// The `GMT0` time zone.
	pub struct Gmt0;
	impl Sealed for Gmt0 {
		const NAME: &'static str = "GMT0";
		const DATA: TzifData = GREENWICH;
	}
	impl TimeZone for Gmt0 {}
	/// The `Greenwich` time zone.
	pub struct Greenwich;
	impl Sealed for Greenwich {
		const NAME: &'static str = "Greenwich";
		const DATA: TzifData = GREENWICH;
	}
	impl TimeZone for Greenwich {}
	/// The `HST` time zone.
	pub struct Hst;
	impl Sealed for Hst {
		const NAME: &'static str = "HST";
		const DATA: TzifData = HST;
	}
	impl TimeZone for Hst {}
	/// The `Hongkong` time zone.
	pub struct Hongkong;
	impl Sealed for Hongkong {
		const NAME: &'static str = "Hongkong";
		const DATA: TzifData = HONGKONG;
	}
	impl TimeZone for Hongkong {}
	/// The `Iceland` time zone.
	pub struct Iceland;
	impl Sealed for Iceland {
		const NAME: &'static str = "Iceland";
		const DATA: TzifData = ICELAND;
	}
	impl TimeZone for Iceland {}
	pub mod Indian {
		use super::*;
		/// The `Indian/Antananarivo` time zone.
		pub struct Antananarivo;
		impl Sealed for Antananarivo {
			const NAME: &'static str = "Indian/Antananarivo";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Antananarivo {}
		/// The `Indian/Chagos` time zone.
		pub struct Chagos;
		impl Sealed for Chagos {
			const NAME: &'static str = "Indian/Chagos";
			const DATA: TzifData = INDIAN_CHAGOS;
		}
		impl TimeZone for Chagos {}
		/// The `Indian/Christmas` time zone.
		pub struct Christmas;
		impl Sealed for Christmas {
			const NAME: &'static str = "Indian/Christmas";
			const DATA: TzifData = ASIA_VIENTIANE;
		}
		impl TimeZone for Christmas {}
		/// The `Indian/Cocos` time zone.
		pub struct Cocos;
		impl Sealed for Cocos {
			const NAME: &'static str = "Indian/Cocos";
			const DATA: TzifData = ASIA_RANGOON;
		}
		impl TimeZone for Cocos {}
		/// The `Indian/Comoro` time zone.
		pub struct Comoro;
		impl Sealed for Comoro {
			const NAME: &'static str = "Indian/Comoro";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Comoro {}
		/// The `Indian/Kerguelen` time zone.
		pub struct Kerguelen;
		impl Sealed for Kerguelen {
			const NAME: &'static str = "Indian/Kerguelen";
			const DATA: TzifData = INDIAN_KERGUELEN;
		}
		impl TimeZone for Kerguelen {}
		/// The `Indian/Mahe` time zone.
		pub struct Mahe;
		impl Sealed for Mahe {
			const NAME: &'static str = "Indian/Mahe";
			const DATA: TzifData = ASIA_MUSCAT;
		}
		impl TimeZone for Mahe {}
		/// The `Indian/Maldives` time zone.
		pub struct Maldives;
		impl Sealed for Maldives {
			const NAME: &'static str = "Indian/Maldives";
			const DATA: TzifData = INDIAN_KERGUELEN;
		}
		impl TimeZone for Maldives {}
		/// The `Indian/Mauritius` time zone.
		pub struct Mauritius;
		impl Sealed for Mauritius {
			const NAME: &'static str = "Indian/Mauritius";
			const DATA: TzifData = INDIAN_MAURITIUS;
		}
		impl TimeZone for Mauritius {}
		/// The `Indian/Mayotte` time zone.
		pub struct Mayotte;
		impl Sealed for Mayotte {
			const NAME: &'static str = "Indian/Mayotte";
			const DATA: TzifData = INDIAN_MAYOTTE;
		}
		impl TimeZone for Mayotte {}
		/// The `Indian/Reunion` time zone.
		pub struct Reunion;
		impl Sealed for Reunion {
			const NAME: &'static str = "Indian/Reunion";
			const DATA: TzifData = ASIA_MUSCAT;
		}
		impl TimeZone for Reunion {}
	}
	/// The `Iran` time zone.
	pub struct Iran;
	impl Sealed for Iran {
		const NAME: &'static str = "Iran";
		const DATA: TzifData = IRAN;
	}
	impl TimeZone for Iran {}
	/// The `Israel` time zone.
	pub struct Israel;
	impl Sealed for Israel {
		const NAME: &'static str = "Israel";
		const DATA: TzifData = ISRAEL;
	}
	impl TimeZone for Israel {}
	/// The `Jamaica` time zone.
	pub struct Jamaica;
	impl Sealed for Jamaica {
		const NAME: &'static str = "Jamaica";
		const DATA: TzifData = JAMAICA;
	}
	impl TimeZone for Jamaica {}
	/// The `Japan` time zone.
	pub struct Japan;
	impl Sealed for Japan {
		const NAME: &'static str = "Japan";
		const DATA: TzifData = JAPAN;
	}
	impl TimeZone for Japan {}
	/// The `Kwajalein` time zone.
	pub struct Kwajalein;
	impl Sealed for Kwajalein {
		const NAME: &'static str = "Kwajalein";
		const DATA: TzifData = KWAJALEIN;
	}
	impl TimeZone for Kwajalein {}
	/// The `Libya` time zone.
	pub struct Libya;
	impl Sealed for Libya {
		const NAME: &'static str = "Libya";
		const DATA: TzifData = LIBYA;
	}
	impl TimeZone for Libya {}
	/// The `MET` time zone.
	pub struct Met;
	impl Sealed for Met {
		const NAME: &'static str = "MET";
		const DATA: TzifData = MET;
	}
	impl TimeZone for Met {}
	/// The `MST` time zone.
	pub struct Mst;
	impl Sealed for Mst {
		const NAME: &'static str = "MST";
		const DATA: TzifData = MST;
	}
	impl TimeZone for Mst {}
	/// The `MST7MDT` time zone.
	pub struct Mst7Mdt;
	impl Sealed for Mst7Mdt {
		const NAME: &'static str = "MST7MDT";
		const DATA: TzifData = MST_7_MDT;
	}
	impl TimeZone for Mst7Mdt {}
	pub mod Mexico {
		use super::*;
		/// The `Mexico/BajaNorte` time zone.
		pub struct BajaNorte;
		impl Sealed for BajaNorte {
			const NAME: &'static str = "Mexico/BajaNorte";
			const DATA: TzifData = MEXICO_BAJA_NORTE;
		}
		impl TimeZone for BajaNorte {}
		/// The `Mexico/BajaSur` time zone.
		pub struct BajaSur;
		impl Sealed for BajaSur {
			const NAME: &'static str = "Mexico/BajaSur";
			const DATA: TzifData = MEXICO_BAJA_SUR;
		}
		impl TimeZone for BajaSur {}
		/// The `Mexico/General` time zone.
		pub struct General;
		impl Sealed for General {
			const NAME: &'static str = "Mexico/General";
			const DATA: TzifData = MEXICO_GENERAL;
		}
		impl TimeZone for General {}
	}
	/// The `NZ` time zone.
	pub struct Nz;
	impl Sealed for Nz {
		const NAME: &'static str = "NZ";
		const DATA: TzifData = NZ;
	}
	impl TimeZone for Nz {}
	/// The `NZ-CHAT` time zone.
	pub struct NzChat;
	impl Sealed for NzChat {
		const NAME: &'static str = "NZ-CHAT";
		const DATA: TzifData = NZ_CHAT;
	}
	impl TimeZone for NzChat {}
	/// The `Navajo` time zone.
	pub struct Navajo;
	impl Sealed for Navajo {
		const NAME: &'static str = "Navajo";
		const DATA: TzifData = US_MOUNTAIN;
	}
	impl TimeZone for Navajo {}
	/// The `PRC` time zone.
	pub struct Prc;
	impl Sealed for Prc {
		const NAME: &'static str = "PRC";
		const DATA: TzifData = PRC;
	}
	impl TimeZone for Prc {}
	/// The `PST8PDT` time zone.
	pub struct Pst8Pdt;
	impl Sealed for Pst8Pdt {
		const NAME: &'static str = "PST8PDT";
		const DATA: TzifData = PST_8_PDT;
	}
	impl TimeZone for Pst8Pdt {}
	pub mod Pacific {
		use super::*;
		/// The `Pacific/Apia` time zone.
		pub struct Apia;
		impl Sealed for Apia {
			const NAME: &'static str = "Pacific/Apia";
			const DATA: TzifData = PACIFIC_APIA;
		}
		impl TimeZone for Apia {}
		/// The `Pacific/Auckland` time zone.
		pub struct Auckland;
		impl Sealed for Auckland {
			const NAME: &'static str = "Pacific/Auckland";
			const DATA: TzifData = NZ;
		}
		impl TimeZone for Auckland {}
		/// The `Pacific/Bougainville` time zone.
		pub struct Bougainville;
		impl Sealed for Bougainville {
			const NAME: &'static str = "Pacific/Bougainville";
			const DATA: TzifData = PACIFIC_BOUGAINVILLE;
		}
		impl TimeZone for Bougainville {}
		/// The `Pacific/Chatham` time zone.
		pub struct Chatham;
		impl Sealed for Chatham {
			const NAME: &'static str = "Pacific/Chatham";
			const DATA: TzifData = NZ_CHAT;
		}
		impl TimeZone for Chatham {}
		/// The `Pacific/Chuuk` time zone.
		pub struct Chuuk;
		impl Sealed for Chuuk {
			const NAME: &'static str = "Pacific/Chuuk";
			const DATA: TzifData = PACIFIC_YAP;
		}
		impl TimeZone for Chuuk {}
		/// The `Pacific/Easter` time zone.
		pub struct Easter;
		impl Sealed for Easter {
			const NAME: &'static str = "Pacific/Easter";
			const DATA: TzifData = CHILE_EASTER_ISLAND;
		}
		impl TimeZone for Easter {}
		/// The `Pacific/Efate` time zone.
		pub struct Efate;
		impl Sealed for Efate {
			const NAME: &'static str = "Pacific/Efate";
			const DATA: TzifData = PACIFIC_EFATE;
		}
		impl TimeZone for Efate {}
		/// The `Pacific/Enderbury` time zone.
		pub struct Enderbury;
		impl Sealed for Enderbury {
			const NAME: &'static str = "Pacific/Enderbury";
			const DATA: TzifData = PACIFIC_ENDERBURY;
		}
		impl TimeZone for Enderbury {}
		/// The `Pacific/Fakaofo` time zone.
		pub struct Fakaofo;
		impl Sealed for Fakaofo {
			const NAME: &'static str = "Pacific/Fakaofo";
			const DATA: TzifData = PACIFIC_FAKAOFO;
		}
		impl TimeZone for Fakaofo {}
		/// The `Pacific/Fiji` time zone.
		pub struct Fiji;
		impl Sealed for Fiji {
			const NAME: &'static str = "Pacific/Fiji";
			const DATA: TzifData = PACIFIC_FIJI;
		}
		impl TimeZone for Fiji {}
		/// The `Pacific/Funafuti` time zone.
		pub struct Funafuti;
		impl Sealed for Funafuti {
			const NAME: &'static str = "Pacific/Funafuti";
			const DATA: TzifData = PACIFIC_WALLIS;
		}
		impl TimeZone for Funafuti {}
		/// The `Pacific/Galapagos` time zone.
		pub struct Galapagos;
		impl Sealed for Galapagos {
			const NAME: &'static str = "Pacific/Galapagos";
			const DATA: TzifData = PACIFIC_GALAPAGOS;
		}
		impl TimeZone for Galapagos {}
		/// The `Pacific/Gambier` time zone.
		pub struct Gambier;
		impl Sealed for Gambier {
			const NAME: &'static str = "Pacific/Gambier";
			const DATA: TzifData = PACIFIC_GAMBIER;
		}
		impl TimeZone for Gambier {}
		/// The `Pacific/Guadalcanal` time zone.
		pub struct Guadalcanal;
		impl Sealed for Guadalcanal {
			const NAME: &'static str = "Pacific/Guadalcanal";
			const DATA: TzifData = PACIFIC_PONAPE;
		}
		impl TimeZone for Guadalcanal {}
		/// The `Pacific/Guam` time zone.
		pub struct Guam;
		impl Sealed for Guam {
			const NAME: &'static str = "Pacific/Guam";
			const DATA: TzifData = PACIFIC_SAIPAN;
		}
		impl TimeZone for Guam {}
		/// The `Pacific/Honolulu` time zone.
		pub struct Honolulu;
		impl Sealed for Honolulu {
			const NAME: &'static str = "Pacific/Honolulu";
			const DATA: TzifData = US_HAWAII;
		}
		impl TimeZone for Honolulu {}
		/// The `Pacific/Johnston` time zone.
		pub struct Johnston;
		impl Sealed for Johnston {
			const NAME: &'static str = "Pacific/Johnston";
			const DATA: TzifData = US_HAWAII;
		}
		impl TimeZone for Johnston {}
		/// The `Pacific/Kanton` time zone.
		pub struct Kanton;
		impl Sealed for Kanton {
			const NAME: &'static str = "Pacific/Kanton";
			const DATA: TzifData = PACIFIC_ENDERBURY;
		}
		impl TimeZone for Kanton {}
		/// The `Pacific/Kiritimati` time zone.
		pub struct Kiritimati;
		impl Sealed for Kiritimati {
			const NAME: &'static str = "Pacific/Kiritimati";
			const DATA: TzifData = PACIFIC_KIRITIMATI;
		}
		impl TimeZone for Kiritimati {}
		/// The `Pacific/Kosrae` time zone.
		pub struct Kosrae;
		impl Sealed for Kosrae {
			const NAME: &'static str = "Pacific/Kosrae";
			const DATA: TzifData = PACIFIC_KOSRAE;
		}
		impl TimeZone for Kosrae {}
		/// The `Pacific/Kwajalein` time zone.
		pub struct Kwajalein;
		impl Sealed for Kwajalein {
			const NAME: &'static str = "Pacific/Kwajalein";
			const DATA: TzifData = KWAJALEIN;
		}
		impl TimeZone for Kwajalein {}
		/// The `Pacific/Majuro` time zone.
		pub struct Majuro;
		impl Sealed for Majuro {
			const NAME: &'static str = "Pacific/Majuro";
			const DATA: TzifData = PACIFIC_WALLIS;
		}
		impl TimeZone for Majuro {}
		/// The `Pacific/Marquesas` time zone.
		pub struct Marquesas;
		impl Sealed for Marquesas {
			const NAME: &'static str = "Pacific/Marquesas";
			const DATA: TzifData = PACIFIC_MARQUESAS;
		}
		impl TimeZone for Marquesas {}
		/// The `Pacific/Midway` time zone.
		pub struct Midway;
		impl Sealed for Midway {
			const NAME: &'static str = "Pacific/Midway";
			const DATA: TzifData = US_SAMOA;
		}
		impl TimeZone for Midway {}
		/// The `Pacific/Nauru` time zone.
		pub struct Nauru;
		impl Sealed for Nauru {
			const NAME: &'static str = "Pacific/Nauru";
			const DATA: TzifData = PACIFIC_NAURU;
		}
		impl TimeZone for Nauru {}
		/// The `Pacific/Niue` time zone.
		pub struct Niue;
		impl Sealed for Niue {
			const NAME: &'static str = "Pacific/Niue";
			const DATA: TzifData = PACIFIC_NIUE;
		}
		impl TimeZone for Niue {}
		/// The `Pacific/Norfolk` time zone.
		pub struct Norfolk;
		impl Sealed for Norfolk {
			const NAME: &'static str = "Pacific/Norfolk";
			const DATA: TzifData = PACIFIC_NORFOLK;
		}
		impl TimeZone for Norfolk {}
		/// The `Pacific/Noumea` time zone.
		pub struct Noumea;
		impl Sealed for Noumea {
			const NAME: &'static str = "Pacific/Noumea";
			const DATA: TzifData = PACIFIC_NOUMEA;
		}
		impl TimeZone for Noumea {}
		/// The `Pacific/Pago_Pago` time zone.
		pub struct PagoPago;
		impl Sealed for PagoPago {
			const NAME: &'static str = "Pacific/Pago_Pago";
			const DATA: TzifData = US_SAMOA;
		}
		impl TimeZone for PagoPago {}
		/// The `Pacific/Palau` time zone.
		pub struct Palau;
		impl Sealed for Palau {
			const NAME: &'static str = "Pacific/Palau";
			const DATA: TzifData = PACIFIC_PALAU;
		}
		impl TimeZone for Palau {}
		/// The `Pacific/Pitcairn` time zone.
		pub struct Pitcairn;
		impl Sealed for Pitcairn {
			const NAME: &'static str = "Pacific/Pitcairn";
			const DATA: TzifData = PACIFIC_PITCAIRN;
		}
		impl TimeZone for Pitcairn {}
		/// The `Pacific/Pohnpei` time zone.
		pub struct Pohnpei;
		impl Sealed for Pohnpei {
			const NAME: &'static str = "Pacific/Pohnpei";
			const DATA: TzifData = PACIFIC_PONAPE;
		}
		impl TimeZone for Pohnpei {}
		/// The `Pacific/Ponape` time zone.
		pub struct Ponape;
		impl Sealed for Ponape {
			const NAME: &'static str = "Pacific/Ponape";
			const DATA: TzifData = PACIFIC_PONAPE;
		}
		impl TimeZone for Ponape {}
		/// The `Pacific/Port_Moresby` time zone.
		pub struct PortMoresby;
		impl Sealed for PortMoresby {
			const NAME: &'static str = "Pacific/Port_Moresby";
			const DATA: TzifData = PACIFIC_YAP;
		}
		impl TimeZone for PortMoresby {}
		/// The `Pacific/Rarotonga` time zone.
		pub struct Rarotonga;
		impl Sealed for Rarotonga {
			const NAME: &'static str = "Pacific/Rarotonga";
			const DATA: TzifData = PACIFIC_RAROTONGA;
		}
		impl TimeZone for Rarotonga {}
		/// The `Pacific/Saipan` time zone.
		pub struct Saipan;
		impl Sealed for Saipan {
			const NAME: &'static str = "Pacific/Saipan";
			const DATA: TzifData = PACIFIC_SAIPAN;
		}
		impl TimeZone for Saipan {}
		/// The `Pacific/Samoa` time zone.
		pub struct Samoa;
		impl Sealed for Samoa {
			const NAME: &'static str = "Pacific/Samoa";
			const DATA: TzifData = US_SAMOA;
		}
		impl TimeZone for Samoa {}
		/// The `Pacific/Tahiti` time zone.
		pub struct Tahiti;
		impl Sealed for Tahiti {
			const NAME: &'static str = "Pacific/Tahiti";
			const DATA: TzifData = PACIFIC_TAHITI;
		}
		impl TimeZone for Tahiti {}
		/// The `Pacific/Tarawa` time zone.
		pub struct Tarawa;
		impl Sealed for Tarawa {
			const NAME: &'static str = "Pacific/Tarawa";
			const DATA: TzifData = PACIFIC_WALLIS;
		}
		impl TimeZone for Tarawa {}
		/// The `Pacific/Tongatapu` time zone.
		pub struct Tongatapu;
		impl Sealed for Tongatapu {
			const NAME: &'static str = "Pacific/Tongatapu";
			const DATA: TzifData = PACIFIC_TONGATAPU;
		}
		impl TimeZone for Tongatapu {}
		/// The `Pacific/Truk` time zone.
		pub struct Truk;
		impl Sealed for Truk {
			const NAME: &'static str = "Pacific/Truk";
			const DATA: TzifData = PACIFIC_YAP;
		}
		impl TimeZone for Truk {}
		/// The `Pacific/Wake` time zone.
		pub struct Wake;
		impl Sealed for Wake {
			const NAME: &'static str = "Pacific/Wake";
			const DATA: TzifData = PACIFIC_WALLIS;
		}
		impl TimeZone for Wake {}
		/// The `Pacific/Wallis` time zone.
		pub struct Wallis;
		impl Sealed for Wallis {
			const NAME: &'static str = "Pacific/Wallis";
			const DATA: TzifData = PACIFIC_WALLIS;
		}
		impl TimeZone for Wallis {}
		/// The `Pacific/Yap` time zone.
		pub struct Yap;
		impl Sealed for Yap {
			const NAME: &'static str = "Pacific/Yap";
			const DATA: TzifData = PACIFIC_YAP;
		}
		impl TimeZone for Yap {}
	}
	/// The `Poland` time zone.
	pub struct Poland;
	impl Sealed for Poland {
		const NAME: &'static str = "Poland";
		const DATA: TzifData = POLAND;
	}
	impl TimeZone for Poland {}
	/// The `Portugal` time zone.
	pub struct Portugal;
	impl Sealed for Portugal {
		const NAME: &'static str = "Portugal";
		const DATA: TzifData = PORTUGAL;
	}
	impl TimeZone for Portugal {}
	/// The `ROC` time zone.
	pub struct Roc;
	impl Sealed for Roc {
		const NAME: &'static str = "ROC";
		const DATA: TzifData = ROC;
	}
	impl TimeZone for Roc {}
	/// The `ROK` time zone.
	pub struct Rok;
	impl Sealed for Rok {
		const NAME: &'static str = "ROK";
		const DATA: TzifData = ROK;
	}
	impl TimeZone for Rok {}
	/// The `Singapore` time zone.
	pub struct Singapore;
	impl Sealed for Singapore {
		const NAME: &'static str = "Singapore";
		const DATA: TzifData = SINGAPORE;
	}
	impl TimeZone for Singapore {}
	/// The `Turkey` time zone.
	pub struct Turkey;
	impl Sealed for Turkey {
		const NAME: &'static str = "Turkey";
		const DATA: TzifData = TURKEY;
	}
	impl TimeZone for Turkey {}
	/// The `UCT` time zone.
	pub struct Uct;
	impl Sealed for Uct {
		const NAME: &'static str = "UCT";
		const DATA: TzifData = ZULU;
	}
	impl TimeZone for Uct {}
	pub mod Us {
		use super::*;
		/// The `US/Alaska` time zone.
		pub struct Alaska;
		impl Sealed for Alaska {
			const NAME: &'static str = "US/Alaska";
			const DATA: TzifData = US_ALASKA;
		}
		impl TimeZone for Alaska {}
		/// The `US/Aleutian` time zone.
		pub struct Aleutian;
		impl Sealed for Aleutian {
			const NAME: &'static str = "US/Aleutian";
			const DATA: TzifData = US_ALEUTIAN;
		}
		impl TimeZone for Aleutian {}
		/// The `US/Arizona` time zone.
		pub struct Arizona;
		impl Sealed for Arizona {
			const NAME: &'static str = "US/Arizona";
			const DATA: TzifData = US_ARIZONA;
		}
		impl TimeZone for Arizona {}
		/// The `US/Central` time zone.
		pub struct Central;
		impl Sealed for Central {
			const NAME: &'static str = "US/Central";
			const DATA: TzifData = US_CENTRAL;
		}
		impl TimeZone for Central {}
		/// The `US/East-Indiana` time zone.
		pub struct EastIndiana;
		impl Sealed for EastIndiana {
			const NAME: &'static str = "US/East-Indiana";
			const DATA: TzifData = US_EAST_INDIANA;
		}
		impl TimeZone for EastIndiana {}
		/// The `US/Eastern` time zone.
		pub struct Eastern;
		impl Sealed for Eastern {
			const NAME: &'static str = "US/Eastern";
			const DATA: TzifData = US_EASTERN;
		}
		impl TimeZone for Eastern {}
		/// The `US/Hawaii` time zone.
		pub struct Hawaii;
		impl Sealed for Hawaii {
			const NAME: &'static str = "US/Hawaii";
			const DATA: TzifData = US_HAWAII;
		}
		impl TimeZone for Hawaii {}
		/// The `US/Indiana-Starke` time zone.
		pub struct IndianaStarke;
		impl Sealed for IndianaStarke {
			const NAME: &'static str = "US/Indiana-Starke";
			const DATA: TzifData = US_INDIANA_STARKE;
		}
		impl TimeZone for IndianaStarke {}
		/// The `US/Michigan` time zone.
		pub struct Michigan;
		impl Sealed for Michigan {
			const NAME: &'static str = "US/Michigan";
			const DATA: TzifData = US_MICHIGAN;
		}
		impl TimeZone for Michigan {}
		/// The `US/Mountain` time zone.
		pub struct Mountain;
		impl Sealed for Mountain {
			const NAME: &'static str = "US/Mountain";
			const DATA: TzifData = US_MOUNTAIN;
		}
		impl TimeZone for Mountain {}
		/// The `US/Pacific` time zone.
		pub struct Pacific;
		impl Sealed for Pacific {
			const NAME: &'static str = "US/Pacific";
			const DATA: TzifData = US_PACIFIC;
		}
		impl TimeZone for Pacific {}
		/// The `US/Samoa` time zone.
		pub struct Samoa;
		impl Sealed for Samoa {
			const NAME: &'static str = "US/Samoa";
			const DATA: TzifData = US_SAMOA;
		}
		impl TimeZone for Samoa {}
	}
	/// The `UTC` time zone.
	pub struct Utc;
	impl Sealed for Utc {
		const NAME: &'static str = "UTC";
		const DATA: TzifData = ZULU;
	}
	impl TimeZone for Utc {}
	/// The `Universal` time zone.
	pub struct Universal;
	impl Sealed for Universal {
		const NAME: &'static str = "Universal";
		const DATA: TzifData = ZULU;
	}
	impl TimeZone for Universal {}
	/// The `W-SU` time zone.
	pub struct WSu;
	impl Sealed for WSu {
		const NAME: &'static str = "W-SU";
		const DATA: TzifData = W_SU;
	}
	impl TimeZone for WSu {}
	/// The `WET` time zone.
	pub struct Wet;
	impl Sealed for Wet {
		const NAME: &'static str = "WET";
		const DATA: TzifData = WET;
	}
	impl TimeZone for Wet {}
	/// The `Zulu` time zone.
	pub struct Zulu;
	impl Sealed for Zulu {
		const NAME: &'static str = "Zulu";
		const DATA: TzifData = ZULU;
	}
	impl TimeZone for Zulu {}
}
