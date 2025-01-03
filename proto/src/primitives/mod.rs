pub mod api_hal;

macro_rules! primitive_enum {
	(
		$(
			$variant:ident($value:expr, $struct:path)
		),* $(,)?
	) => {
		#[allow(non_camel_case_types)]
		#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
		pub enum PrimitiveID {
			$(
				$variant = $value,
			)*
		}

		#[allow(non_camel_case_types)]
		#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
		pub enum PrimitiveParameter {
			$(
				$variant($struct),
			)*
		}

		impl From<PrimitiveID> for PrimitiveParameter {
			fn from(primitive_id: PrimitiveID) -> Self {
				match primitive_id {
					$(
						PrimitiveID::$variant => PrimitiveParameter::$variant(<$struct>::default()),
					)*
				}
			}
		}

		impl From<PrimitiveParameter> for PrimitiveID {
			fn from(primitive_parameter: PrimitiveParameter) -> Self {
				match primitive_parameter {
					$(
						PrimitiveParameter::$variant(_) => PrimitiveID::$variant,
					)*
				}
			}
		}
	};
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct NoParameter;

primitive_enum! {
	// Copied from natalie-dect-h/SrcHeaders/Phoenix/RosPrimitiv.h
	DUMMY_DATA(0x0000, NoParameter),                                                 //Phoenix
	KEY_MESSAGE(0x0001, NoParameter),                                                //Phoenix
	KEY_RELEASE(0x0002, NoParameter),                                                //Phoenix
	KEY_PRESS(0x0003, NoParameter),                                                  //Phoenix
	TIMEOUT(0x0004, NoParameter),                                                    //Phoenix
	HELLO_STATE_ind(0x0005, NoParameter),                                            //Phoenix
	GOODBYE_STATE_ind(0x0006, NoParameter),                                          //Phoenix
	INITTASK(0x0007, NoParameter),                                                   //Phoenix
	TERMINATETASK(0x0008, NoParameter),                                              //Phoenix
	TEST_PRIMITIVE(0x0009, NoParameter),                                             //Phoenix
	ROS_PM_WAKEUP_ind(0x000A, NoParameter),                                          //Phoenix
	ROS_PM_SUSPEND_ind(0x000B, NoParameter),                                         //Phoenix
	SCL_INIT_TIMEOUT(0x0010, NoParameter),                                           //SUOTA Client
	SCL_INTERVAL_TIMEOUT(0x0011, NoParameter),                                       //SUOTA Client
	SCL_OPERATION_TIMEOUT(0x0012, NoParameter),                                      //SUOTA Client
	SCL_RESET_TIMEOUT(0x0013, NoParameter),                                          //SUOTA Client
	LHC_OPEN_REQ(0x0014, NoParameter),                                               //LdsHttpClient
	LHC_OPEN_CFM(0x0015, NoParameter),                                               //LdsHttpClient
	LHC_CLOSE_REQ(0x0016, NoParameter),                                              //LdsHttpClient
	LHC_CLOSE_IND(0x0017, NoParameter),                                              //LdsHttpClient
	LHC_GET_FILE_REQ(0x0018, NoParameter),                                           //LdsHttpClient
	LHC_GET_FILE_CFM(0x0019, NoParameter),                                           //LdsHttpClient
	LHC_RESUME_REQ(0x001A, NoParameter),                                             //LdsHttpClient
	SW_NWK_GENEVENOT_LT_EVENT_REQ(0x001B, NoParameter),                              //IGenEveNot
	SW_NWK_GENEVENOT_LT_EVENT_IND(0x001C, NoParameter),                              //IGenEveNot
	SW_NWK_GENEVENOT_LT_UT_EVENT_IND(0x001D, NoParameter),                           //IGenEveNot
	SW_NWK_GENEVENOT_UT_EVENT_REQ(0x001E, NoParameter),                              //IGenEveNot
	SW_NWK_GENEVENOT_UT_EVENT_IND(0x001F, NoParameter),                              //IGenEveNot
	SW_NWK_LAS_RELEASE_CON_REQ(0x0020, NoParameter),                                 //Nwk Las Task
	SW_NWK_LAS_RELEASE_CON_CFM(0x0021, NoParameter),                                 //Nwk Las Task
	LM_SORT_REQ(0x0022, NoParameter),                                                //IListManager
	LM_SORT_CFM(0x0023, NoParameter),                                                //IListManager
	AM_ERROR_ind(0x0024, NoParameter),                                               //AudioManager
	AM_LOG_ind(0x0025, NoParameter),                                                 //AudioManager
	FP_LC_DEBUG__MAC_CO_DATA_IND_CRC_ERROR(0x0026, NoParameter),                     //DECT Minigap Fp
	NWK_SWITCH_SWDATA_DEBUG(0x0027, NoParameter),                                    //NwkSwitch DB
	NWK_SWITCH_SWCLIENTDATA_DEBUG(0x0028, NoParameter),                              //NwkSwitch DB
	NWK_SWITCH_DB_DEBUG(0x0029, NoParameter),                                        //NwkSwitch DB
	NWK_SWITCH_PIDATA_DEBUG(0x002A, NoParameter),                                    //NwkSwitch PiAdmin
	SW_AUDIO_CONNECT_REQ(0x002B, NoParameter),                                       //IFpSwAudio
	SW_AUDIO_CONNECT_CFM(0x002C, NoParameter),                                       //IFpSwAudio
	SW_AUDIO_DISCONNECT_REQ(0x002D, NoParameter),                                    //IFpSwAudio
	SW_AUDIO_DISCONNECT_CFM(0x002E, NoParameter),                                    //IFpSwAudio
	SW_AUDIO_DEBUG_CONNECT_CFM(0x002F, NoParameter),                                 //IFpSwAudio
	SW_AUDIO_DEBUG_DISCONNECT_CFM(0x0030, NoParameter),                              //IFpSwAudio
	SW_NWK_PROXY_OPEN_REQ(0x0031, NoParameter),                                      //ISwNwkProxy
	SW_NWK_PROXY_CONNECT_IND(0x0032, NoParameter),                                   //ISwNwkProxy
	SW_NWK_PROXY_CLOSE_REQ(0x0033, NoParameter),                                     //ISwNwkProxy
	SW_NWK_PROXY_REJECT_IND(0x0034, NoParameter),                                    //ISwNwkProxy
	SW_NWK_PROXY_RELEASE_IND(0x0035, NoParameter),                                   //ISwNwkProxy
	SW_NWK_SYNC_TIME_DATE_REQ(0x0036, NoParameter),                                  //ISwNwkPi
	SW_NWK_CLSS_PROPRIETARY_REQ(0x0037, NoParameter),                                //ISwNwkPi
	SW_NWK_CLSS_FP_GENEVENT_IND(0x0038, NoParameter),                                //ISwNwkPi
	SW_NWK_CLSS_GENEVENT_REQ(0x0039, NoParameter),                                   //ISwNwkPi
	EE_WRITE_DEFAULT_req(0x003A, NoParameter),                                       //RtxEai
	SW_NWK_SYNC_TIME_DATE_IND(0x003B, NoParameter),                                  //ISwNwkPi
	SW_NWK_CLSS_PP_GENEVENT_REQ(0x003C, NoParameter),                                //ISwNwkPi
	SW_NWK_CLSS_PP_GENEVENT_IND(0x003D, NoParameter),                                //ISwNwkPi
	SW_NWK_CLSS_REQ(0x003E, NoParameter),                                            //ISwNwkPi
	SW_NWK_CLSS_IND(0x003F, NoParameter),                                            //ISwNwkPi
	SW_NWK_CC_UT_SETUP_REQ(0x0040, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_UT_SETUP_CFM(0x0041, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_UT_CALL_PROC_IND(0x0042, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_UT_ALERT_IND(0x0043, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_UT_CONNECT_IND(0x0044, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_UT_CONNECT_RES(0x0045, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_LT_SETUP_IND(0x0046, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_LT_SETUP_RES(0x0047, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_LT_CALL_PROC_REQ(0x0048, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_LT_ALERT_REQ(0x0049, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_LT_CONNECT_REQ(0x004A, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_LT_CONNECT_CFM(0x004B, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_UT_SETUP_IND(0x004C, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_UT_ALERT_REQ(0x004D, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_UT_CONNECT_REQ(0x004E, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_UT_CONNECT_CFM(0x004F, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_LT_SETUP_REQ(0x0050, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_LT_ALERT_IND(0x0051, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_LT_CONNECT_IND(0x0052, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_LT_CONNECT_RES(0x0053, NoParameter),                                   //ISwNwkCc
	SW_NWK_CC_UT_INFO_IND(0x0054, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_UT_INFO_REQ(0x0055, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_INFO_IND(0x0056, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_INFO_REQ(0x0057, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_RELEASE_REQ(0x0058, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_RELEASE_CFM(0x0059, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_RELEASE_IND(0x005A, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_RELEASE_RES(0x005B, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_REJECT_IND(0x005C, NoParameter),                                       //ISwNwkCc
	TEST_CMD_req(0x005D, NoParameter),                                               //ProdTest
	SW_NWK_CC_REJECT_REQ(0x005E, NoParameter),                                       //ISwNwkCc
	SW_NWK_CC_IWU_INFO_IWUTOIWU_IND(0x005F, NoParameter),                            //ISwNwkCc
	SW_NWK_CC_IWU_INFO_IWUTOIWU_REQ(0x0060, NoParameter),                            //ISwNwkCc
	SW_NWK_CC_ULE_IWU_INFO_IND(0x0061, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_ULE_IWU_INFO_REQ(0x0062, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_IWU_INFO_IND(0x0063, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_IWU_INFO_REQ(0x0064, NoParameter),                                     //ISwNwkCc
	SW_NWK_CC_MODIFY_CODEC_REQ(0x0065, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_MODIFY_CODEC_CFM(0x0066, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_MODIFY_CODEC_IND(0x0067, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_MODIFY_CODEC_RES(0x0068, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_IWU_INFO_CODEC_IND(0x0069, NoParameter),                               //ISwNwkCc
	SW_NWK_CC_IWU_INFO_SLOT_CHANGE_IND(0x006A, NoParameter),                         //ISwNwkCc
	SW_NWK_CC_MODIFY_ULE_SERVICE_REQ(0x006B, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_MODIFY_ULE_SERVICE_CFM(0x006C, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_MODIFY_ULE_SERVICE_IND(0x006D, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_MODIFY_ULE_SERVICE_RES(0x006E, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_LT_CALL_WAITING_REQ(0x006F, NoParameter),                              //ISwNwkCc
	SW_NWK_CC_LT_CALL_WAITING_CFM(0x0070, NoParameter),                              //ISwNwkCc
	SW_NWK_CC_UT_CALL_WAITING_IND(0x0071, NoParameter),                              //ISwNwkCc
	SW_NWK_CC_UT_CALL_WAITING_RES(0x0072, NoParameter),                              //ISwNwkCc
	SW_NWK_CC_UT_END_CALL_REQ(0x0073, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_LT_END_CALL_IND(0x0074, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_LT_END_CALL_RES(0x0075, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_UT_TRANSFER_CALL_REQ(0x0076, NoParameter),                             //ISwNwkCc
	SW_NWK_CC_UT_TOGGLE_CALL_REQ(0x0077, NoParameter),                               //ISwNwkCc
	SW_NWK_CC_LT_HOLD_CALL_REQ(0x0078, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_LT_HOLD_CALL_CFM(0x0079, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_LT_RETRIEVE_CALL_REQ(0x007A, NoParameter),                             //ISwNwkCc
	SW_NWK_CC_LT_RETRIEVE_CALL_CFM(0x007B, NoParameter),                             //ISwNwkCc
	SW_NWK_CC_UT_HOLD_CALL_REQ(0x007C, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_UT_RETRIEVE_CALL_REQ(0x007D, NoParameter),                             //ISwNwkCc
	SW_NWK_CC_UT_MPTY_REQ(0x007E, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_MPTY_IND(0x007F, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_MPTY_RES(0x0080, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_MPTY_REQ(0x0081, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_MPTY_CFM(0x0082, NoParameter),                                      //ISwNwkCc
	SW_NWK_CC_LT_MPTY_PREP_IND(0x0083, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_LT_MPTY_PREP_RES(0x0084, NoParameter),                                 //ISwNwkCc
	SW_NWK_CC_LT_MPTY_END_IND(0x0085, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_LT_MPTY_END_RES(0x0086, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_UT_SECOND_CALL_INTERCOM_REQ(0x0087, NoParameter),                      //ISwNwkCc
	SW_NWK_CC_UT_SECOND_CALL_EXTERNAL_REQ(0x0088, NoParameter),                      //ISwNwkCc
	SW_NWK_CC_UT_EXT_CALL_DEFLECTION_REQ(0x0089, NoParameter),                       //ISwNwkCc
	SW_NWK_CC_UT_INT_CALL_DEFLECTION_REQ(0x008A, NoParameter),                       //ISwNwkCc
	SW_NWK_CC_UT_CALL_SCREENING_ACCEPT_REQ(0x008B, NoParameter),                     //ISwNwkCc
	SW_NWK_CC_UT_CALL_SCREENING_INTERCEPT_REQ(0x008C, NoParameter),                  //ISwNwkCc
	SW_NWK_CC_LT_CALL_SCREENING_ACCEPT_IND(0x008D, NoParameter),                     //ISwNwkCc
	SW_NWK_CC_LT_CALL_SCREENING_INTERCEPT_IND(0x008E, NoParameter),                  //ISwNwkCc
	SW_NWK_CC_LT_CALL_SCREENING_SETUP_IND(0x008F, NoParameter),                      //ISwNwkCc
	SW_NWK_CC_LT_CALL_SCREENING_SETUP_RES(0x0090, NoParameter),                      //ISwNwkCc
	SW_NWK_CC_RP_APP_DATA_REQ(0x0091, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_RP_APP_DATA_IND(0x0092, NoParameter),                                  //ISwNwkCc
	SW_NWK_CC_CONNECT_END_POINTS_REQ(0x0093, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_CONNECT_END_POINTS_CFM(0x0094, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_NEW_OTHER_PARTY_IND(0x0095, NoParameter),                              //ISwNwkCc
	SW_NWK_CC_SELECTED_ADPCM_IND(0x0096, NoParameter),                               //ISwNwkCc
	SW_NWK_CC_LT_RECONNECT_AUDIO_REQ(0x0097, NoParameter),                           //ISwNwkCc
	SW_NWK_CC_LT_DISCONNECT_AUDIO_REQ(0x0098, NoParameter),                          //ISwNwkCc
	SW_NWK_SS_UT_SETUP_REQ(0x0099, NoParameter),                                     //ISwNwkCiss
	SW_NWK_SS_UT_SETUP_IND(0x009A, NoParameter),                                     //ISwNwkCiss
	SW_NWK_SS_UT_RELEASE_REQ(0x009B, NoParameter),                                   //ISwNwkCiss
	SW_NWK_SS_UT_RELEASE_IND(0x009C, NoParameter),                                   //ISwNwkCiss
	SW_NWK_SS_UT_FACILITY_IND(0x009D, NoParameter),                                  //ISwNwkCiss
	SW_NWK_SS_UT_FACILITY_REQ(0x009E, NoParameter),                                  //ISwNwkCiss
	SW_NWK_SS_LT_SETUP_IND(0x009F, NoParameter),                                     //ISwNwkCiss
	SW_NWK_SS_LT_SETUP_REQ(0x00A0, NoParameter),                                     //ISwNwkCiss
	SW_NWK_SS_LT_RELEASE_REQ(0x00A1, NoParameter),                                   //ISwNwkCiss
	SW_NWK_SS_LT_RELEASE_IND(0x00A2, NoParameter),                                   //ISwNwkCiss
	SW_NWK_SS_LT_FACILITY_IND(0x00A3, NoParameter),                                  //ISwNwkCiss
	SW_NWK_SS_LT_FACILITY_REQ(0x00A4, NoParameter),                                  //ISwNwkCiss
	SW_NWK_SS_RP_APP_DATA_REQ(0x00A5, NoParameter),                                  //ISwNwkCiss
	SW_NWK_SS_RP_APP_DATA_IND(0x00A6, NoParameter),                                  //ISwNwkCiss
	API_CVM_FP_PROJECT_DSP_TONE_END_IND(0x00A7, NoParameter),                        //Api Project
	API_CVM_FP_PROJECT_DSP_MIDI_END_IND(0x00A8, NoParameter),                        //Api Project
	API_FP_ULP_TEST_CMD(0x00A9, NoParameter),                                        //Natalie V3 CVM
	TEST_MAC_BURST_FIG31_REQ(0x00AA, NoParameter),                                   //DECT Minigap Fp
	DRV_MNG_DFC_ind(0x00AB, NoParameter),                                            //DevDrvMng
	EE_INVALIDATE_CACHE(0x00AC, NoParameter),                                        //EEPROM
	EE_INTSIM_READ(0x00AD, NoParameter),                                             //EEPROM
	EE_INTSIM_WRITE(0x00AE, NoParameter),                                            //EEPROM
	RTX_EAI_MAIL2TX(0x00AF, NoParameter),                                            //RtxEai
	FWU_DIRECT(0x00B0, NoParameter),                                                 //FWU Manager
	FWU_TRACE(0x00B1, NoParameter),                                                  //FWU Manager
	DL_DATA_REQ(0x00B2, NoParameter),                                                //FWU Manager
	DL_DATA_IND(0x00B3, NoParameter),                                                //FWU Manager
	PROXY_CONNECT_IND(0x00B4, NoParameter),                                          //FWU Client
	PROXY_DISCONNECT_IND(0x00B5, NoParameter),                                       //FWU Client
	M_DATA_DATA_READY(0x00B6, NoParameter),                                          //DECT Minigap Data
	M_DATA_DATA_IND(0x00B7, NoParameter),                                            //DECT Minigap Data
	M_DATA_DATA_REQ(0x00B8, NoParameter),                                            //DECT Minigap Data
	COLA_DISABLE_REQ(0x00B9, NoParameter),                                           //COLA Task
	COLA_DISABLE_CFM(0x00BA, NoParameter),                                           //COLA Task
	COLA_INIT_REQ(0x00BB, NoParameter),                                              //COLA Task
	COLA_INIT_CFM(0x00BC, NoParameter),                                              //COLA Task
	COLA_START_DELAY_TIMEOUT(0x00BD, NoParameter),                                   //COLA
	API_IMAGE_ACTIVATE_TIMEOUT(0x00BE, NoParameter),                                 //Api Image Control
	API_HAL_LED_TIMEOUT(0x00BF, NoParameter),                                        //Api HAL
	VES_DEFRAG(0x00C0, NoParameter),                                                 //VES
	RAMBUS_POLL_TIMEOUT(0x00C1, NoParameter),                                        //RAMBUS Driver
	SC_UPDATE_PLL(0x00C2, NoParameter),                                              //System Clock Manager
	SPI_DRIVER_TX_DONE(0x00C3, NoParameter),                                         //Spi Driver
	SPI_DRIVER_RX_DATA_READY(0x00C4, NoParameter),                                   //Spi Driver
	SLS_RX_DATA_READY(0x00C5, NoParameter),                                          //Sercom Linux Socket
	SLS_SOCKET_READY(0x00C6, NoParameter),                                           //Sercom Linux Socket
	SLC_RX_DATA_READY(0x00C7, NoParameter),                                          //Sercom Linux Char
	UART_TX_DONE(0x00C8, NoParameter),                                               //Uart Driver
	UART_RX_DATA_READY(0x00C9, NoParameter),                                         //Uart Driver
	UART_PMM_TIMEOUT(0x00CA, NoParameter),                                           //Uart Driver
	FP_BASIC_BEARER_IND(0x0100, NoParameter),                                        //DECT Minigap Common
	FP_BASIC_BEARER_HANDOVER_IND(0x0101, NoParameter),                               //DECT Minigap Common
	FP_BASIC_CONNECTION_HANDOVER_IND(0x0102, NoParameter),                           //DECT Minigap Common
	TIMEOUT_RUN_MODULATION_CALIBRATION(0x0103, NoParameter),                         //DECT Minigap Common
	FP_MAC_DEBUG_MODCAL(0x0104, NoParameter),                                        //DECT Minigap Common
	FP_MAC_CON_IND(0x0105, NoParameter),                                             //DECT Minigap Common
	FP_MAC_DIS_IND(0x0106, NoParameter),                                             //DECT Minigap Common
	FP_MAC_DIS_REQ(0x0107, NoParameter),                                             //DECT Minigap Common
	MAC_NO_EMISSION_MODE_STOP_IND(0x0108, NoParameter),                              //DECT Minigap Common
	MAC_NO_EMISSION_MODE_PENDING_IND(0x0109, NoParameter),                           //DECT Minigap Common
	MAC_NO_EMISSION_MODE_ACTIVE_IND(0x010A, NoParameter),                            //DECT Minigap Common
	MAC_NO_EMISSION_MODE_STOP_REQ(0x010B, NoParameter),                              //DECT Minigap Common
	MAC_ENABLE_NO_EMISSION_MODE(0x010C, NoParameter),                                //DECT Minigap Common
	MAC_DISABLE_NO_EMISSION_MODE(0x010D, NoParameter),                               //DECT Minigap Common
	TIMEOUT_CC06(0x010E, NoParameter),                                               //DECT Minigap Common
	PP_AUTHENTICATE_ACC(0x010F, NoParameter),                                        //DECT Minigap Common
	FP_BEARER_RELEASE_IND(0x0110, NoParameter),                                      //DECT Minigap Common
	FP_BROADCAST_QUALITY_IND(0x0111, NoParameter),                                   //DECT Minigap Common
	FP_TIMEOUT_USA_MOVE_DUMMY(0x0112, NoParameter),                                  //DECT Minigap Common
	FP_MAC_DEBUG_DUMMY_STARTED(0x0113, NoParameter),                                 //DECT Minigap Common
	FP_MAC_DEBUG_N210_TIMEOUT(0x0114, NoParameter),                                  //DECT Minigap Common
	FP_BROADCAST_STOPPED_IND(0x0115, NoParameter),                                   //DECT Minigap Common
	FP_ULP_DLC_ENC_KEY_IND(0x0116, NoParameter),                                     //DECT Minigap Common
	FP_ULP_ERROR_IND(0x0117, NoParameter),                                           //DECT Minigap Common
	AUTH_START_REQ(0x0118, NoParameter),                                             //DECT Minigap Common
	AUTH_LOOP_IND(0x0119, NoParameter),                                              //DECT Minigap Common
	AUTH_COMPLETION_IND(0x011A, NoParameter),                                        //DECT Minigap Common
	AUTH_DSAA2_FIRST_STEP_REQ(0x011B, NoParameter),                                  //DECT Minigap Common
	AUTH_DSAA2_FIRST_STEP_COMPLETE_IND(0x011C, NoParameter),                         //DECT Minigap Common
	AUTH_DSAA2_SECOND_STEP_REQ(0x011D, NoParameter),                                 //DECT Minigap Common
	GF_CHANNEL_DATA_IND(0x011E, NoParameter),                                        //DECT Minigap Common
	LU10_DEBUG_RX(0x011F, NoParameter),                                              //DECT Minigap Common
	LU10_DEBUG_TX(0x0120, NoParameter),                                              //DECT Minigap Common
	LU10_DEBUG_TX_RETRANS(0x0121, NoParameter),                                      //DECT Minigap Common
	LU10_DEBUG_TX_SENDPACKET(0x0122, NoParameter),                                   //DECT Minigap Common
	LU10_DEBUG_RX_FU10C_ACK_IN_EMPTY_FU10A(0x0123, NoParameter),                     //DECT Minigap Common
	LU10_DEBUG_RX_RETRANS_OF_ALREADY_ACK_PACKET(0x0124, NoParameter),                //DECT Minigap Common
	LU10_DEBUG_RX_ACK(0x0125, NoParameter),                                          //DECT Minigap Common
	LU10_DEBUG_RX_OUT_OF_SEQ_PDU(0x0126, NoParameter),                               //DECT Minigap Common
	LU10_DEBUG_RX_FU10A(0x0127, NoParameter),                                        //DECT Minigap Common
	LU10_DATA_READY(0x0128, NoParameter),                                            //DECT Minigap Common
	LU10_DATA_IND(0x0129, NoParameter),                                              //DECT Minigap Common
	LU10_DEBUG_TX_ACK_IN_FU10C(0x012A, NoParameter),                                 //DECT Minigap Common
	LU10_DEBUG_TX_IDLE_ACK_IN_FU10C(0x012B, NoParameter),                            //DECT Minigap Common
	LU10_ENABLE_HIGH_CLOCK(0x012C, NoParameter),                                     //DECT Minigap Common
	LU10_DISABLE_HIGH_CLOCK(0x012D, NoParameter),                                    //DECT Minigap Common
	LU10_DEBUG_RX_2ND_INFO_FIELD(0x012E, NoParameter),                               //DECT Minigap Common
	FP_MAC_DEBUG_DUMMY_NOT_POSSIBLE(0x012F, NoParameter),                            //DECT Minigap Common
	FP_MAC_CON_RES(0x0130, NoParameter),                                             //DECT Minigap Common
	PRNG_DEBUG_ENTROPY(0x0131, NoParameter),                                         //DECT Minigap Common
	PRNG_DEBUG_EMPTY(0x0132, NoParameter),                                           //DECT Minigap Common
	PRNG_DEBUG_REPLACE(0x0133, NoParameter),                                         //DECT Minigap Common
	LU10_DEBUG_CONNECTION_CLOSED(0x0134, NoParameter),                               //DECT Minigap Common
	LU10_DEBUG_RX_FU10C_ACK(0x0135, NoParameter),                                    //DECT Minigap Common
	FP_ULP_DLC_LEGACY_ENC_KEY_IND(0x0136, NoParameter),                              //DECT Minigap Common
	FP_ADVANCED_BEARER_IND(0x0200, NoParameter),                                     //DECT Minigap Fp
	FP_ADVANCED_BEARER_HANDOVER_IND(0x0201, NoParameter),                            //DECT Minigap Fp
	FP_BEARER_QUALITY_IND(0x0202, NoParameter),                                      //DECT Minigap Fp
	OLD__FP_BEARER_RELEASE_IND(0x0203, NoParameter),                                 //DECT Minigap Fp
	FP_BEARER_ATTRIBUTES_IND(0x0204, NoParameter),                                   //DECT Minigap Fp
	FP_BEARER_ATTRIBUTES_CFM(0x0205, NoParameter),                                   //DECT Minigap Fp
	FP_CS_DATA_SEGMENT_IND(0x0206, NoParameter),                                     //DECT Minigap Fp
	OLD__FP_BROADCAST_QUALITY_IND(0x0207, NoParameter),                              //DECT Minigap Fp
	FP_MULTIFRAME_SYNC_IND(0x0208, NoParameter),                                     //DECT Minigap Fp
	FP_TIMEOUT_T218(0x0209, NoParameter),                                            //DECT Minigap Fp
	FP_MAC_STOP_REQ(0x020A, NoParameter),                                            //DECT Minigap Fp
	FP_MAC_DEBUG_DBC_NOT_POSSIBLE(0x020B, NoParameter),                              //DECT Minigap Fp
	FP_MAC_DEBUG_DBC_RSSI_3_NOT_POSSIBLE(0x020C, NoParameter),                       //DECT Minigap Fp
	FP_CC_DEBUG_NO_USER_AUTH_NOT_FIRST_INSTANCE(0x020D, NoParameter),                //DECT Minigap Fp
	FP_CC_DEBUG_NO_USER_AUTH_TBR_22_ENABLED(0x020E, NoParameter),                    //DECT Minigap Fp
	FP_CC_DEBUG_NO_USER_AUTH_END_RUNNING(0x020F, NoParameter),                       //DECT Minigap Fp
	FP_RSSI_SCAN_IND(0x0210, NoParameter),                                           //DECT Minigap Fp
	FP_TIMEOUT_RSSI_SCAN(0x0211, NoParameter),                                       //DECT Minigap Fp
	FP_HANDSET_ENROLLED(0x0212, NoParameter),                                        //DECT Minigap Fp
	FP_HANDSET_DISENROLLED(0x0213, NoParameter),                                     //DECT Minigap Fp
	FP_START_MAC(0x0214, NoParameter),                                               //DECT Minigap Fp
	FP_START_MAC_PCM_SYNC(0x0215, NoParameter),                                      //DECT Minigap Fp
	FP_BEARER_RELEASE_REQ(0x0216, NoParameter),                                      //DECT Minigap Fp
	FP_BEARER_ENC_REQ(0x0217, NoParameter),                                          //DECT Minigap Fp
	FP_MAC_CF_DATA_REQ(0x0218, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DATA_SEGMENT_IND(0x0219, NoParameter),                                     //DECT Minigap Fp
	FP_CS_DATA_SEGMENT_REQ(0x021A, NoParameter),                                     //DECT Minigap Fp
	FP_BEARER_REQ(0x021B, NoParameter),                                              //DECT Minigap Fp
	FP_BEARER_START_QUALITY_MEASURAMENT_REQ(0x021C, NoParameter),                    //DECT Minigap Fp
	FP_BEARER_AUDIO_TO_BUFFER_2(0x021D, NoParameter),                                //DECT Minigap Fp
	FP_BEARER_CFM(0x021E, NoParameter),                                              //DECT Minigap Fp
	FP_CS_DATA_DTR_OK_IND(0x021F, NoParameter),                                      //DECT Minigap Fp
	FP_CS_DATA_DTR_ERROR_IND(0x0220, NoParameter),                                   //DECT Minigap Fp
	FP_CF_DATA_DTR_OK_IND(0x0221, NoParameter),                                      //DECT Minigap Fp
	FP_CF_DATA_DTR_ERROR_IND(0x0222, NoParameter),                                   //DECT Minigap Fp
	FP_BEARER_ENC_EKS_IND(0x0223, NoParameter),                                      //DECT Minigap Fp
	FP_BROADCAST_CHANNEL_REQ(0x0224, NoParameter),                                   //DECT Minigap Fp
	FP_BROADCAST_QUALITY_IND_REQUEST_DBC(0x0225, NoParameter),                       //DECT Minigap Fp
	FP_MAC_SET_ENROLL_BIT__EASY_PAIRING(0x0226, NoParameter),                        //DECT Minigap Fp
	FP_MNCL_UNITDATA_CBI_REQ(0x0227, NoParameter),                                   //DECT Minigap Fp
	FP_MNCL_UNIT_DATA_REQ__SEND_BASE_NAME(0x0228, NoParameter),                      //DECT Minigap Fp
	FP_MAC_SET_EXTENDED_HIGHER_LAYER_INF_PART_2(0x0229, NoParameter),                //DECT Minigap Fp
	MAC_NO_EMISSION_SET_PREFFERED_CARRIER(0x022A, NoParameter),                      //DECT Minigap Fp
	FP_BEARER_DUAL_SLOT_REQ(0x022B, NoParameter),                                    //DECT Minigap Fp
	FP_BROADCAST_QUALITY_IND_DEBUG(0x022C, NoParameter),                             //DECT Minigap Fp
	FP_BROADCAST_QUALITY_IND_DEBUG_2(0x022D, NoParameter),                           //DECT Minigap Fp
	FP_BROADCAST_QUALITY_SET_NEW_RSSI_LIMIT(0x022E, NoParameter),                    //DECT Minigap Fp
	FP_BROADCAST_QUALITY_SET_NEW_RSSI_LIMIT_2(0x022F, NoParameter),                  //DECT Minigap Fp
	FP_RSSI_SCAN_IND_JAPAN_CHANNEL_CHECK(0x0230, NoParameter),                       //DECT Minigap Fp
	FP_RSSI_SCAN_IND_JAPAN_DEBUG(0x0231, NoParameter),                               //DECT Minigap Fp
	FP_MAC_DEBUG_DUMMY_TIME_CHECK_FAILED(0x0232, NoParameter),                       //DECT Minigap Fp
	TIMEOUT_TI_0(0x0233, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_1(0x0234, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_2(0x0235, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_3(0x0236, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_4(0x0237, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_5(0x0238, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_8(0x0239, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_9(0x023A, NoParameter),                                               //DECT Minigap Fp
	TIMEOUT_TI_10(0x023B, NoParameter),                                              //DECT Minigap Fp
	TIMEOUT_TI_11(0x023C, NoParameter),                                              //DECT Minigap Fp
	TIMEOUT_TI_12(0x023D, NoParameter),                                              //DECT Minigap Fp
	TIMEOUT_TI_13(0x023E, NoParameter),                                              //DECT Minigap Fp
	FP_BEARER_ENC_EKS_STOP_IND(0x023F, NoParameter),                                 //DECT Minigap Fp
	FP_BEARER_ENC_EKS_IND_EARLY_ENCRYPTION(0x0240, NoParameter),                     //DECT Minigap Fp
	FP_MAC_ENC_EKS_IND_EARLY_ENCRYPTION(0x0241, NoParameter),                        //DECT Minigap Fp
	FP_MAC_CO_DATA_IND(0x0242, NoParameter),                                         //DECT Minigap Fp
	FP_MAC_CO_DATA_REQ(0x0243, NoParameter),                                         //DECT Minigap Fp
	FP_MAC_CO_DTR_IND(0x0244, NoParameter),                                          //DECT Minigap Fp
	FP_MAC_ENC_KEY_REQ(0x0245, NoParameter),                                         //DECT Minigap Fp
	FP_MAC_ENC_EKS_IND(0x0246, NoParameter),                                         //DECT Minigap Fp
	FP_MAC_MOD_REQ(0x0247, NoParameter),                                             //DECT Minigap Fp
	FP_MAC_MOD_IND(0x0248, NoParameter),                                             //DECT Minigap Fp
	FP_MAC_MOD_CFM(0x0249, NoParameter),                                             //DECT Minigap Fp
	FP_MAC_SET_ENROLL_BIT(0x024A, NoParameter),                                      //DECT Minigap Fp
	FP_MAC_CLEAR_ENROLL_BIT(0x024B, NoParameter),                                    //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_LOCAL(0x024C, NoParameter),                                 //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_ADPCM(0x024D, NoParameter),                                 //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_SILENCE(0x024E, NoParameter),                               //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_CLIP(0x024F, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_MINIGAP_DATA(0x0250, NoParameter),                          //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_LU10(0x0251, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_CONNECT_BROADCAST_AUDIO(0x0252, NoParameter),                             //DECT Minigap Fp
	FP_TIMEOUT_AUDIO_QUEUE(0x0253, NoParameter),                                     //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_MINIGAP_DATA_FROM_QUEUE(0x0254, NoParameter),               //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_LU10_FROM_QUEUE(0x0255, NoParameter),                       //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_LOCAL_FROM_QUEUE(0x0256, NoParameter),                      //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_ADPCM_FROM_QUEUE(0x0257, NoParameter),                      //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_SILENCE_FROM_QUEUE(0x0258, NoParameter),                    //DECT Minigap Fp
	FP_MAC_CONNECT_AUDIO_CLIP_FROM_QUEUE(0x0259, NoParameter),                       //DECT Minigap Fp
	FP_MAC_PAGE_REQ(0x025A, NoParameter),                                            //DECT Minigap Fp
	FP_MAC_LONG_PAGE_REQ(0x025B, NoParameter),                                       //DECT Minigap Fp
	FP_ALERT_BROADCAST_REQ(0x025C, NoParameter),                                     //DECT Minigap Fp
	FP_MAC_ESCAPE_PT_REQ(0x025D, NoParameter),                                       //DECT Minigap Fp
	FP_XBC_DEBUG_SETUP_FAILED_CRC_1_OTHER(0x025E, NoParameter),                      //DECT Minigap Fp
	FP_BEARER_FORCE_HANDOVER_REQ(0x025F, NoParameter),                               //DECT Minigap Fp
	FP_XBC_DEBUG_SETUP_FAILED_CRC_ADVANCED_ATTRIBUTES(0x0260, NoParameter),          //DECT Minigap Fp
	FP_XBC_DEBUG_ATTRIBUTES_IGNORED_CAUSE_ADPCM_ON(0x0261, NoParameter),             //DECT Minigap Fp
	NO_EMISSION_DEBUG__SYNC_FOUND_IND(0x0262, NoParameter),                          //DECT Minigap Fp
	NO_EMISSION_DEBUG__A_FIELD_FOUND_IND(0x0263, NoParameter),                       //DECT Minigap Fp
	NO_EMISSION_DEBUG__SYNC_FOUND_IND__NO_FREE_INSTANCE(0x0264, NoParameter),        //DECT Minigap Fp
	FP_CCF_CLOSE_DUMMY(0x0265, NoParameter),                                         //DECT Minigap Fp
	FP_BEARER_ENC_KEY_REQ(0x0266, NoParameter),                                      //DECT Minigap Fp
	FP_BEARER_AUDIO_TO_BUFFER(0x0267, NoParameter),                                  //DECT Minigap Fp
	FP_CCF_DEBUG_CLOSE_INSTANCE(0x0268, NoParameter),                                //DECT Minigap Fp
	FP_CCF_DEBUG_NEW_INSTANCE(0x0269, NoParameter),                                  //DECT Minigap Fp
	TIMEOUT_MM_RE_KEYING(0x026A, NoParameter),                                       //DECT Minigap Fp
	FP_TIMEOUT_LCE(0x026B, NoParameter),                                             //DECT Minigap Fp
	FP_TIMEOUT_LAPC(0x026C, NoParameter),                                            //DECT Minigap Fp
	FP_DL_SERVICE_MOD_REQ_CPLANE(0x026D, NoParameter),                               //DECT Minigap Fp
	FP_DL_SERVICE_MOD_CFM_CPLANE(0x026E, NoParameter),                               //DECT Minigap Fp
	FP_DL_SERVICE_MOD_IND_CPLANE(0x026F, NoParameter),                               //DECT Minigap Fp
	FP_DL_SERVICE_MOD_REQ(0x0270, NoParameter),                                      //DECT Minigap Fp
	FP_DL_SERVICE_MOD_CFM(0x0271, NoParameter),                                      //DECT Minigap Fp
	FP_DL_SERVICE_MOD_IND(0x0272, NoParameter),                                      //DECT Minigap Fp
	FP_CC_DEBUG_NEW_INSTANCE(0x0273, NoParameter),                                   //DECT Minigap Fp
	FP_CC_DEBUG_CLOSE_INSTANCE(0x0274, NoParameter),                                 //DECT Minigap Fp
	FP_CISS_DEBUG_CLOSE_INSTANCE(0x0275, NoParameter),                               //DECT Minigap Fp
	FP_CISS_DEBUG_NEW_INSTANCE(0x0276, NoParameter),                                 //DECT Minigap Fp
	FP_CISS_DEBUG_NO_ACTIVE_CISS_CONNECTION(0x0277, NoParameter),                    //DECT Minigap Fp
	FP_LC_DATA_REQ(0x0278, NoParameter),                                             //DECT Minigap Fp
	FP_LC_DATA_IND(0x0279, NoParameter),                                             //DECT Minigap Fp
	FP_LC_RELEASE_REQ(0x027A, NoParameter),                                          //DECT Minigap Fp
	FP_LC_ENCRYPT_CRYPTED_IND(0x027B, NoParameter),                                  //DECT Minigap Fp
	FP_DL_ESTABLISH_IND(0x027C, NoParameter),                                        //DECT Minigap Fp
	FP_DL_DATA_IND(0x027D, NoParameter),                                             //DECT Minigap Fp
	FP_DL_RELEASE_IND(0x027E, NoParameter),                                          //DECT Minigap Fp
	FP_DL_RELEASE_CFM(0x027F, NoParameter),                                          //DECT Minigap Fp
	FP_DL_DATA_REQ(0x0280, NoParameter),                                             //DECT Minigap Fp
	FP_DL_ENCRYPT_IND(0x0281, NoParameter),                                          //DECT Minigap Fp
	FP_DL_RELEASE_REQ(0x0282, NoParameter),                                          //DECT Minigap Fp
	FP_LC_RELEASE_IND(0x0283, NoParameter),                                          //DECT Minigap Fp
	FP_LAPC_DEBUG_CLOSE_INSTANCE(0x0284, NoParameter),                               //DECT Minigap Fp
	FP_LAPC_DEBUG_NEW_INSTANCE(0x0285, NoParameter),                                 //DECT Minigap Fp
	FP_LC_DEBUG_CLOSE_INSTANCE(0x0286, NoParameter),                                 //DECT Minigap Fp
	FP_LC_ENC_KEY_REQ(0x0287, NoParameter),                                          //DECT Minigap Fp
	FP_LC_DEBUG_NEW_INSTANCE(0x0288, NoParameter),                                   //DECT Minigap Fp
	FP_LC_DEBUG_GET_SLOT_TYPE(0x0289, NoParameter),                                  //DECT Minigap Fp
	FP_LC_DEBUG_LONG_SLOT(0x028A, NoParameter),                                      //DECT Minigap Fp
	FP_LC_DEBUG_DOUBLE_SLOT(0x028B, NoParameter),                                    //DECT Minigap Fp
	FP_LC_DEBUG_FULL_SLOT(0x028C, NoParameter),                                      //DECT Minigap Fp
	FP_LCE_DEBUG_CLOSE_INSTANCE(0x028D, NoParameter),                                //DECT Minigap Fp
	FP_LCE_DEBUG_NEW_INSTANCE(0x028E, NoParameter),                                  //DECT Minigap Fp
	FP_LC_CF_TO_CS_REQ(0x028F, NoParameter),                                         //DECT Minigap Fp
	FP_LC_CS_TO_CF_REQ(0x0290, NoParameter),                                         //DECT Minigap Fp
	FP_LCE_CC_RELEASE_LINK_IND(0x0291, NoParameter),                                 //DECT Minigap Fp
	FP_LCE_CC_RELEASE_LINK_REQ(0x0292, NoParameter),                                 //DECT Minigap Fp
	FP_LCE_MM_RELEASE_LINK_IND(0x0293, NoParameter),                                 //DECT Minigap Fp
	FP_LCE_MM_RELEASE_LINK_REQ(0x0294, NoParameter),                                 //DECT Minigap Fp
	FP_LCE_CISS_RELEASE_LINK_REQ(0x0295, NoParameter),                               //DECT Minigap Fp
	FP_LCE_CISS_RELEASE_LINK_IND(0x0296, NoParameter),                               //DECT Minigap Fp
	FP_MNCC_ALERT_REQ(0x0297, NoParameter),                                          //DECT Minigap Fp
	FP_MNCC_ALERT_IND(0x0298, NoParameter),                                          //DECT Minigap Fp
	FP_MNCC_CALL_PROC_REQ(0x0299, NoParameter),                                      //DECT Minigap Fp
	FP_MNCC_SETUP_ACK_REQ(0x029A, NoParameter),                                      //DECT Minigap Fp
	FP_MNCC_CONNECT_REQ(0x029B, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_CONNECT_IND(0x029C, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_CONNECT_CFM(0x029D, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_CONNECT_ACK_REQ(0x029E, NoParameter),                                    //DECT Minigap Fp
	FP_MNCC_HOLD_IND(0x029F, NoParameter),                                           //DECT Minigap Fp
	FP_MNCC_IWU_INFO_REQ(0x02A0, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_IWU_INFO_IND(0x02A1, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_INFO_IND(0x02A2, NoParameter),                                           //DECT Minigap Fp
	FP_MNCC_INFO_REQ(0x02A3, NoParameter),                                           //DECT Minigap Fp
	FP_MNCC_MODIFY_REQ(0x02A4, NoParameter),                                         //DECT Minigap Fp
	FP_MNCC_MODIFY_IND(0x02A5, NoParameter),                                         //DECT Minigap Fp
	FP_MNCC_REJECT_REQ(0x02A6, NoParameter),                                         //DECT Minigap Fp
	FP_MNCC_REJECT_IND(0x02A7, NoParameter),                                         //DECT Minigap Fp
	FP_MNCC_HOLD_ACK_RES(0x02A8, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_HOLD_ACK_IND(0x02A9, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_HOLD_REJECT_IND(0x02AA, NoParameter),                                    //DECT Minigap Fp
	FP_MNCC_HOLD_REJECT_RES(0x02AB, NoParameter),                                    //DECT Minigap Fp
	FP_MNCC_RETRIEVE_ACK_RES(0x02AC, NoParameter),                                   //DECT Minigap Fp
	FP_MNCC_RETRIEVE_ACK_IND(0x02AD, NoParameter),                                   //DECT Minigap Fp
	FP_MNCC_RETRIEVE_REJECT_IND(0x02AE, NoParameter),                                //DECT Minigap Fp
	FP_MNCC_RETRIEVE_REJECT_RES(0x02AF, NoParameter),                                //DECT Minigap Fp
	FP_MNCC_RELEASE_REQ(0x02B0, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_RELEASE_IND(0x02B1, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_RELEASE_CFM(0x02B2, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_RELEASE_RES(0x02B3, NoParameter),                                        //DECT Minigap Fp
	FP_MNCC_RETRIEVE_IND(0x02B4, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_SETUP_REQ(0x02B5, NoParameter),                                          //DECT Minigap Fp
	FP_MNCC_SETUP_IND(0x02B6, NoParameter),                                          //DECT Minigap Fp
	FP_MNCC_FACILITY_REQ(0x02B7, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_FACILITY_IND(0x02B8, NoParameter),                                       //DECT Minigap Fp
	FP_MNCC_SERVICE_CHANGE_RES(0x02B9, NoParameter),                                 //DECT Minigap Fp
	FP_MNCC_MODIFY_CFM_SUCCES(0x02BA, NoParameter),                                  //DECT Minigap Fp
	FP_MNCC_MODIFY_CFM_FAILURE(0x02BB, NoParameter),                                 //DECT Minigap Fp
	FP_MNCC_MODIFY_RES_SUCCES(0x02BC, NoParameter),                                  //DECT Minigap Fp
	FP_MNCC_MODIFY_RES_FAILURE(0x02BD, NoParameter),                                 //DECT Minigap Fp
	FP_MNCC_PROPRIETARY_IND(0x02BE, NoParameter),                                    //DECT Minigap Fp
	FP_MNCC_CLSS_REQ(0x02BF, NoParameter),                                           //DECT Minigap Fp
	FP_MNCC_CLSS_IND(0x02C0, NoParameter),                                           //DECT Minigap Fp
	FP_MNCC_MODIFY_CFM(0x02C1, NoParameter),                                         //DECT Minigap Fp
	FP_XXX_UNUSED_002(0x02C2, NoParameter),                                          //DECT Minigap Fp
	FP_XXX_UNUSED_003(0x02C3, NoParameter),                                          //DECT Minigap Fp
	FP_XXX_UNUSED_004(0x02C4, NoParameter),                                          //DECT Minigap Fp
	FP_MNMM_DETACH_IND(0x02C5, NoParameter),                                         //DECT Minigap Fp
	FP_MNMM_PT_TERM_ACC_RIGHTS_IND(0x02C6, NoParameter),                             //DECT Minigap Fp
	TIMEOUT_ENC_START(0x02C7, NoParameter),                                          //DECT Minigap Fp
	FP_MNSS_SETUP_REQ(0x02C8, NoParameter),                                          //DECT Minigap Fp
	FP_MNSS_FACILITY_REQ(0x02C9, NoParameter),                                       //DECT Minigap Fp
	FP_MNSS_RELEASE_REQ(0x02CA, NoParameter),                                        //DECT Minigap Fp
	FP_MNSS_RELEASE_IND(0x02CB, NoParameter),                                        //DECT Minigap Fp
	FP_MNSS_SETUP_IND(0x02CC, NoParameter),                                          //DECT Minigap Fp
	FP_MNSS_FACILITY_IND(0x02CD, NoParameter),                                       //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_RESUME_REQ(0x02CE, NoParameter),                       //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_RESUME_CFM(0x02CF, NoParameter),                       //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_RESUME_REJ(0x02D0, NoParameter),                       //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_SUSPEND_REQ(0x02D1, NoParameter),                      //DECT Minigap Fp
	FP_ULP_DLC_DTR_IND(0x02D2, NoParameter),                                         //DECT Minigap Fp
	FP_ULP_DLC_DATA_IND(0x02D3, NoParameter),                                        //DECT Minigap Fp
	FP_MAC_ULP_ENABLE_REQ(0x02D4, NoParameter),                                      //DECT Minigap Fp
	FP_ULP_DLC_PAGE_REQ(0x02D5, NoParameter),                                        //DECT Minigap Fp
	FP_ULP_DLC_FREE_DLBUF_REQ(0x02D6, NoParameter),                                  //DECT Minigap Fp
	FP_LLME_CF_TO_CS_REQ(0x02D7, NoParameter),                                       //DECT Minigap Fp
	FP_LLME_CS_TO_CF_REQ(0x02D8, NoParameter),                                       //DECT Minigap Fp
	FP_MM_DATA_REQ(0x02D9, NoParameter),                                             //DECT Minigap Fp
	FP_MM_DATA_IND(0x02DA, NoParameter),                                             //DECT Minigap Fp
	PP_AUTHENTICATE_UAK_REQ(0x02DB, NoParameter),                                    //DECT Minigap Fp
	PP_AUTHENTICATE_REJ(0x02DC, NoParameter),                                        //DECT Minigap Fp
	FT_TERM_ACC_RIGHTS_REQ(0x02DD, NoParameter),                                     //DECT Minigap Fp
	KEY_ALLOCATE_REQ(0x02DE, NoParameter),                                           //DECT Minigap Fp
	KEY_ALLOCATE_CFM(0x02DF, NoParameter),                                           //DECT Minigap Fp
	KEY_ALLOCATE_REJ(0x02E0, NoParameter),                                           //DECT Minigap Fp
	PP_AUTHENTICATE_UAK_REQ__ASSIGN_DEFAULT_KEY(0x02E1, NoParameter),                //DECT Minigap Fp
	FP_EARLY_ENC_NEW_KEY(0x02E2, NoParameter),                                       //DECT Minigap Fp
	FP_MNMM_IDENTITY_REQ(0x02E3, NoParameter),                                       //DECT Minigap Fp
	FP_MNMM_USER_AUTHENTICATE_REQ(0x02E4, NoParameter),                              //DECT Minigap Fp
	FP_MM_PRNG_ADD_ENTROPY(0x02E5, NoParameter),                                     //DECT Minigap Fp
	FP_MNMM_INFO_SUGGEST_REQ(0x02E6, NoParameter),                                   //DECT Minigap Fp
	FP_MNMM_PP_AUTHENTICATE_AC_REQ(0x02E7, NoParameter),                             //DECT Minigap Fp
	FP_MNMM_PP_AUTHENTICATE_UAK_REQ(0x02E8, NoParameter),                            //DECT Minigap Fp
	FP_MNMM_PP_AUTHENTICATE_ACC(0x02E9, NoParameter),                                //DECT Minigap Fp
	FP_MNMM_PP_AUTHENTICATE_REJ(0x02EA, NoParameter),                                //DECT Minigap Fp
	FP_MNMM_FT_TERM_ACC_RIGHTS_REQ(0x02EB, NoParameter),                             //DECT Minigap Fp
	FP_MNMM_FT_TERM_ACC_RIGHTS_REJ(0x02EC, NoParameter),                             //DECT Minigap Fp
	FP_MNMM_FT_TERM_ACC_RIGHTS_ACC(0x02ED, NoParameter),                             //DECT Minigap Fp
	FP_MNMM_LOCATION_UPDATE_REQ(0x02EE, NoParameter),                                //DECT Minigap Fp
	FP_MNMM_CIPHER_REQ(0x02EF, NoParameter),                                         //DECT Minigap Fp
	FP_MNMM_ACCESS_RIGHTS_IND(0x02F0, NoParameter),                                  //DECT Minigap Fp
	FP_MNMM_LOCATE_IND(0x02F1, NoParameter),                                         //DECT Minigap Fp
	FP_MNCL_UNITDATA_REQ(0x02F2, NoParameter),                                       //DECT Minigap Fp
	FP_MNMM_FT_TERM_ACC_RIGHTS_IND_PP_NOT_ENROLLED(0x02F3, NoParameter),             //DECT Minigap Fp
	FP_CISS_DATA_REQ(0x02F4, NoParameter),                                           //DECT Minigap Fp
	FP_CISS_DATA_IND(0x02F5, NoParameter),                                           //DECT Minigap Fp
	FP_CISS_DATA_REQ__AND_LINK_RELEASE(0x02F6, NoParameter),                         //DECT Minigap Fp
	FP_CC_DATA_REQ(0x02F7, NoParameter),                                             //DECT Minigap Fp
	FP_CC_DATA_IND(0x02F8, NoParameter),                                             //DECT Minigap Fp
	FP_CC_DATA_REQ__AND_LINK_RELEASE(0x02F9, NoParameter),                           //DECT Minigap Fp
	FP_CC_ENCRYPT_IND(0x02FA, NoParameter),                                          //DECT Minigap Fp
	FP_CC_ENCRYPT_REJECT_IND(0x02FB, NoParameter),                                   //DECT Minigap Fp
	FP_MM_ENCRYPT_IND(0x02FC, NoParameter),                                          //DECT Minigap Fp
	CVM_FP_REPEATER_ACTIVE_REQ(0x02FD, NoParameter),                                 //DECT Minigap Fp
	FP_BEARER_REJECT_REQ(0x02FE, NoParameter),                                       //DECT Minigap Fp
	FP_ADVANCED_BEARER_UPLINK_CFM(0x02FF, NoParameter),                              //DECT Minigap Fp
	FP_BEARER_CON_CTRL_HANDOVER_REQ(0x0300, NoParameter),                            //DECT Minigap Fp
	FP_BEARER_ACKNOWLEDGED_RELEASE_REQ(0x0301, NoParameter),                         //DECT Minigap Fp
	FP_MAC_DEBUG_BEARER_COUNTERS_INCREASED(0x0302, NoParameter),                     //DECT Minigap Fp
	FP_BEARER_T201_TIMEOUT_IND(0x0303, NoParameter),                                 //DECT Minigap Fp
	FP_BEARER_BAD_UPLINK_SIGNAL_QUALITY_IND(0x0304, NoParameter),                    //DECT Minigap Fp
	FP_ADVANCED_BEARER_UPLINK_IND(0x0305, NoParameter),                              //DECT Minigap Fp
	FP_BEARER_BANDWIDTH_IND(0x0306, NoParameter),                                    //DECT Minigap Fp
	FP_MAC_DEBUG_ASYM_WRONG_NT(0x0307, NoParameter),                                 //DECT Minigap Fp
	FP_MAC_DEBUG_ASYM_WRONG_PMID(0x0308, NoParameter),                               //DECT Minigap Fp
	FP_MAC_DEBUG_ASYM_WRONG_LBN(0x0309, NoParameter),                                //DECT Minigap Fp
	FP_MAC_DEBUG_ASYM_MESSAGE_TYPE(0x030A, NoParameter),                             //DECT Minigap Fp
	FP_CC_DEBUG_NO_USER_AUTH_DOUBLE_SLOT_LOOPBACK(0x030B, NoParameter),              //DECT Minigap Fp
	FP_BMC_DEBUG_MODCAL_SLOT(0x030C, NoParameter),                                   //DECT Minigap Fp
	FP_BMC_DEBUG_MODCAL_SLOT_PREV_FULL(0x030D, NoParameter),                         //DECT Minigap Fp
	FP_BMC_DEBUG_MODCAL_SLOT_PREV_LONG(0x030E, NoParameter),                         //DECT Minigap Fp
	FP_MAC_DFC_REQ(0x030F, NoParameter),                                             //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_START_REQ(0x0310, NoParameter),                            //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_START_CFM(0x0311, NoParameter),                            //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_START_REJ(0x0312, NoParameter),                            //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_STOP_REQ(0x0313, NoParameter),                             //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_STOPPED_IND(0x0314, NoParameter),                          //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_STOP_CFM(0x0315, NoParameter),                             //DECT Minigap Fp
	FP_XXX_UNUSED_001(0x0316, NoParameter),                                          //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_CHANNEL_REQ(0x0317, NoParameter),                          //DECT Minigap Fp
	FP_BEARER_CON_CTRL_HANDOVER_IND(0x0318, NoParameter),                            //DECT Minigap Fp
	FP_MAC_CF_DATA_REQ__COMPLETE_PACKET(0x0319, NoParameter),                        //DECT Minigap Fp
	FP_LC_DEBUG_C_SERVICE_MOD_PENDING(0x031A, NoParameter),                          //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_AUDIO(0x031B, NoParameter),                              //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_CF(0x031C, NoParameter),                            //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_CF(0x031D, NoParameter),                            //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_LOCAL(0x031E, NoParameter),                         //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_LOCAL(0x031F, NoParameter),                         //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_ADPCM_0(0x0320, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_ADPCM_0(0x0321, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_ADPCM_1(0x0322, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_ADPCM_1(0x0323, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_ADPCM_2(0x0324, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_ADPCM_2(0x0325, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_ADPCM_3(0x0326, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_ADPCM_3(0x0327, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_LONG_MUTE(0x0328, NoParameter),                          //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_FULL_MUTE(0x0329, NoParameter),                          //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_TX_START_ADPCM_PENDING(0x032A, NoParameter),                //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_AUDIO(0x032B, NoParameter),                              //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_START_ADPCM_PENDING(0x032C, NoParameter),                //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_CF(0x032D, NoParameter),                            //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_CF(0x032E, NoParameter),                            //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_LOCAL(0x032F, NoParameter),                         //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_LOCAL(0x0330, NoParameter),                         //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_ADPCM_0(0x0331, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_ADPCM_0(0x0332, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_ADPCM_1(0x0333, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_ADPCM_1(0x0334, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_ADPCM_2(0x0335, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_ADPCM_2(0x0336, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_ADPCM_3(0x0337, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_ADPCM_3(0x0338, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_LONG_SCRATCH(0x0339, NoParameter),                       //DECT Minigap Fp
	FP_BMC_DEBUG_CONNECT_RX_FULL_SCRATCH(0x033A, NoParameter),                       //DECT Minigap Fp
	FP_MAC_CF_DEBUG_RX_ALL_CF(0x033B, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_CF_DEBUG_RX_AUDIO_CONNECTED(0x033C, NoParameter),                         //DECT Minigap Fp
	FP_MAC_CF_DEBUG_CRC(0x033D, NoParameter),                                        //DECT Minigap Fp
	FP_MAC_CF_DEBUG_RX_NOT_ALL_CF(0x033E, NoParameter),                              //DECT Minigap Fp
	FP_BMC_DEBUG_BUILD_RX_SLOT(0x033F, NoParameter),                                 //DECT Minigap Fp
	FP_BMC_DEBUG_BUILD_RX_SCAN_SLOT(0x0340, NoParameter),                            //DECT Minigap Fp
	FP_BMC_DEBUG_LOOPBACK(0x0341, NoParameter),                                      //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_TIMEOUT(0x0342, NoParameter),                              //DECT Minigap Fp
	FP_MAC_DEBUG_MAC_MODIFICATION_QUEUED(0x0343, NoParameter),                       //DECT Minigap Fp
	FP_MAC_DEBUG_MAC_MODIFICATION_DEQUEUED(0x0344, NoParameter),                     //DECT Minigap Fp
	FP_CF_DEBUG_DATA_1(0x0345, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_2(0x0346, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_3(0x0347, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_4(0x0348, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_5(0x0349, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_6(0x034A, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_7(0x034B, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_8(0x034C, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_9(0x034D, NoParameter),                                         //DECT Minigap Fp
	FP_CF_DEBUG_DATA_10(0x034E, NoParameter),                                        //DECT Minigap Fp
	FP_CF_DEBUG_DATA_CRC(0x034F, NoParameter),                                       //DECT Minigap Fp
	FP_CF_DEBUG_DATA_CF_FROM_ADPCM_FRAME(0x0350, NoParameter),                       //DECT Minigap Fp
	FP_CF_DEBUG_DATA_SAVE_FRAME(0x0351, NoParameter),                                //DECT Minigap Fp
	FP_CF_DEBUG_DATA_COMPARE_FRAME(0x0352, NoParameter),                             //DECT Minigap Fp
	FP_CF_DEBUG_DATA_FIRST_BYTE_MATCH(0x0353, NoParameter),                          //DECT Minigap Fp
	FP_CF_DEBUG_DATA_NO_MATCH(0x0354, NoParameter),                                  //DECT Minigap Fp
	FP_CF_DEBUG_DATA_FRAME_MATCH(0x0355, NoParameter),                               //DECT Minigap Fp
	FP_CF_DEBUG_DATA_MORE_THAN_1_MATCH(0x0356, NoParameter),                         //DECT Minigap Fp
	FP_MAC_PING_IND(0x0357, NoParameter),                                            //DECT Minigap Fp
	FP_MM_DEBUG_INVALID_MMEI(0x0358, NoParameter),                                   //DECT Minigap Fp
	FP_MM_DEBUG_INVALID_TI(0x0359, NoParameter),                                     //DECT Minigap Fp
	FP_MM_DEBUG_CLEAR_STATE(0x035A, NoParameter),                                    //DECT Minigap Fp
	FP_MM_DEBUG_ALLOC_MM_INST(0x035B, NoParameter),                                  //DECT Minigap Fp
	HS_CRADLE_IND(0x035C, NoParameter),                                              //Natalie V3 CVM
	HS_UNCRADLE_IND(0x035D, NoParameter),                                            //Natalie V3 CVM
	FP_MAC_ESCAPE_PT_SYNC(0x035E, NoParameter),                                      //DECT Minigap Fp
	FP_MNCC_HOLD_REQ(0x035F, NoParameter),                                           //DECT Minigap Fp
	FP_MNCC_RETRIEVE_REQ(0x0360, NoParameter),                                       //DECT Minigap Fp
	FP_MAC_JAPAN_DECT_SCAN_COMPLETE_IND(0x0361, NoParameter),                        //DECT Minigap Fp
	FP_MAC_JAPAN_DECT_ACTIVE_SCAN_COMPLETE_IND(0x0362, NoParameter),                 //DECT Minigap Fp
	FP_MAC_JAPAN_DECT_SCAN_DEBUG_IND(0x0363, NoParameter),                           //DECT Minigap Fp
	FP_MAC_JAPAN_DECT_SCAN_TIMEOUT(0x0364, NoParameter),                             //DECT Minigap Fp
	FP_MAC_DEBUG_ACCESS_REQ_BLOCKED_CARRIER(0x0365, NoParameter),                    //DECT Minigap Fp
	FP_MAC_DEBUG_ACCESS_REQ_RSSI_TOO_HIGH(0x0366, NoParameter),                      //DECT Minigap Fp
	FP_MAC_DEBUG_DBC_CLOSED_RSSI(0x0367, NoParameter),                               //DECT Minigap Fp
	FP_MAC_ASYM_BROADCAST_CHANNEL_CFM(0x0368, NoParameter),                          //DECT Minigap Fp
	FP_MAC_DUMMY_MOVE_TIMEOUT(0x0369, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_DEBUG_DUMMY_STOPPED(0x036A, NoParameter),                                 //DECT Minigap Fp
	FP_MAC_DEBUG_WRONG_SLOT_1(0x036B, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_DEBUG_WRONG_SLOT_2(0x036C, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_ENC_EKS_IND_REKEYING(0x036D, NoParameter),                                //DECT Minigap Fp
	FP_LC_ENCRYPT_CRYPTED_IND_REKEYING(0x036E, NoParameter),                         //DECT Minigap Fp
	FP_DL_ENCRYPT_IND_REKEYING(0x036F, NoParameter),                                 //DECT Minigap Fp
	FP_CC_ENCRYPT_IND_REKEYING(0x0370, NoParameter),                                 //DECT Minigap Fp
	FP_MM_ENCRYPT_IND_REKEYING(0x0371, NoParameter),                                 //DECT Minigap Fp
	FP_CC_ENCRYPT_REJECT_IND_REKEYING(0x0372, NoParameter),                          //DECT Minigap Fp
	FP_TBR22_START_AUTH(0x0373, NoParameter),                                        //DECT Minigap Fp
	FP_MAC_LOCKED_IND(0x0374, NoParameter),                                          //DECT Minigap Fp
	FP_BEARER_SYNC_REQ(0x0375, NoParameter),                                         //DECT Minigap Fp
	FP_MAC_UNLOCKED_IND(0x0376, NoParameter),                                        //DECT Minigap Fp
	TIMEOUT_FP_SERVICE_TASK(0x0377, NoParameter),                                    //DECT Minigap Fp
	FP_MAC_STATUS_UPDATE(0x0378, NoParameter),                                       //DECT Minigap Fp
	FP_RECOMMENDED_OTHER_BEARER_IND(0x0379, NoParameter),                            //DECT Minigap Fp
	FP_DUMMY_OR_CL_BEARER_POSITION_IND(0x037A, NoParameter),                         //DECT Minigap Fp
	FP_BROADCAST_QUALITY_IND_SLOT_USED(0x037B, NoParameter),                         //DECT Minigap Fp
	FP_DUMMY_OR_CL_BEARER_MARKER_IND(0x037C, NoParameter),                           //DECT Minigap Fp
	FP_BEARER_SYNC_CORRUPT_BROADCAST_CHANNEL_IND(0x037D, NoParameter),               //DECT Minigap Fp
	FP_SYNC_DEBUG_WRONG_RFPI(0x037E, NoParameter),                                   //DECT Minigap Fp
	FP_SYNC_DEBUG_WRONG_FRAME_NUMBER(0x037F, NoParameter),                           //DECT Minigap Fp
	FP_SYNC_DEBUG_WRONG_PRIMRECVCARRIER(0x0380, NoParameter),                        //DECT Minigap Fp
	FP_SYNC_DEBUG_WRONG_SLOT(0x0381, NoParameter),                                   //DECT Minigap Fp
	FP_SYNC_DEBUG_WRONG_MULTIFRAME_NUMBER(0x0382, NoParameter),                      //DECT Minigap Fp
	FP_OTHER_BEARER_IND(0x0383, NoParameter),                                        //DECT Minigap Fp
	FP_CL_BEARER_POSITION_IND(0x0384, NoParameter),                                  //DECT Minigap Fp
	FP_SYNC_QUALITY_IND(0x0385, NoParameter),                                        //DECT Minigap Fp
	FP_BEARER_HANDOVER_REQ(0x0386, NoParameter),                                     //DECT Minigap Fp
	FP_MAC_ODD_MODE_REQ(0x0387, NoParameter),                                        //DECT Minigap Fp
	FP_MAC_ODD_MODE_CFM(0x0388, NoParameter),                                        //DECT Minigap Fp
	FP_MAC_ODD_MODE_CAPABILITY_IND(0x0389, NoParameter),                             //DECT Minigap Fp
	FP_PT_ESCAPE_CHANGE_TO_ODD_MODE_IND(0x038A, NoParameter),                        //DECT Minigap Fp
	FP_SYNC_DEBUG_BACKUP_SELECTED(0x038B, NoParameter),                              //DECT Minigap Fp
	FP_MAC_SLAVE_FOUND_IND(0x038C, NoParameter),                                     //DECT Minigap Fp
	FP_SYNC_BACKUP_MASTER_STATUS_IND(0x038D, NoParameter),                           //DECT Minigap Fp
	FP_SYNC_DEBUG_ODD_MODE_REQ(0x038E, NoParameter),                                 //DECT Minigap Fp
	FP_SYNC_DEBUG_SLAVE_LOST(0x038F, NoParameter),                                   //DECT Minigap Fp
	FP_SYNC_DEBUG_ODD_MODE_TIMEOUT(0x0390, NoParameter),                             //DECT Minigap Fp
	FP_SYNC_TRIG_DEBUG_UPDATE_IND(0x0391, NoParameter),                              //DECT Minigap Fp
	SYNC_TIMEOUT(0x0392, NoParameter),                                               //DECT Minigap Fp
	FP_MAC_SLAVE_ASYM_FOUND_IND(0x0393, NoParameter),                                //DECT Minigap Fp
	FP_BEARER_SET_LOCAL_MODE_REQ(0x0394, NoParameter),                               //DECT Minigap Fp
	FP_SYNC_TRIG_SLAVE_DBG_INFO_IND(0x0395, NoParameter),                            //DECT Minigap Fp
	FP_BMC_DEBUG_ENABLE_ENC(0x0396, NoParameter),                                    //DECT Minigap Fp
	FP_SYNC_DEBUG_NEW_SLAVE(0x0397, NoParameter),                                    //DECT Minigap Fp
	FP_PT_ESCAPE_CHANGE_TO_EVEN_MODE_IND(0x0398, NoParameter),                       //DECT Minigap Fp
	FP_MAC_EVEN_MODE_REQ(0x0399, NoParameter),                                       //DECT Minigap Fp
	FP_MAC_EVEN_MODE_CFM(0x039A, NoParameter),                                       //DECT Minigap Fp
	FP_MAC_SEARCH_TIMEOUT_IND(0x039B, NoParameter),                                  //DECT Minigap Fp
	FP_TIMEOUT_LCE_SHORT_PARTIAL_RELEASE(0x039C, NoParameter),                       //DECT Minigap Fp
	FP_CISS_DATA_IND__CLMS(0x039D, NoParameter),                                     //DECT Minigap Fp
	FP_SYNC_FREERUNNING_IND(0x039E, NoParameter),                                    //DECT Minigap Fp
	FP_SYNC_DEBUG_RELOCK_WRONG_FRAME_NUMBER(0x039F, NoParameter),                    //DECT Minigap Fp
	FP_SYNC_DEBUG_RELOCK_WRONG_SLOT(0x03A0, NoParameter),                            //DECT Minigap Fp
	FP_SYNC_MASTER_BACKUP_FAILOVER_REQ(0x03A1, NoParameter),                         //DECT Minigap Fp
	FP_SYNC_DEBUG_IGNORE_NEW_SOURCE(0x03A2, NoParameter),                            //DECT Minigap Fp
	FP_SYNC_NEW_SOURCE_FOUND_IND(0x03A3, NoParameter),                               //DECT Minigap Fp
	FP_SYNC_DEBUG_EVEN_MODE_REQ(0x03A4, NoParameter),                                //DECT Minigap Fp
	FP_SYNC_DEBUG_RELOCK_MISSING_INFO(0x03A5, NoParameter),                          //DECT Minigap Fp
	FP_SYNC_DEBUG_RELOCK_NOT_MASTER(0x03A6, NoParameter),                            //DECT Minigap Fp
	FP_MAC_DEBUG_NO_LONG_SLOT_ON_SLOT_11(0x03A7, NoParameter),                       //DECT Minigap Fp
	FP_MAC_DEBUG_LONG_SLOT_SECONH_HALF_NOT_FREE(0x03A8, NoParameter),                //DECT Minigap Fp
	FP_MAC_CON_STAT_ENABLE_REQ(0x03A9, NoParameter),                                 //DECT Minigap Fp
	FP_MAC_CON_STAT_DISABLE_REQ(0x03AA, NoParameter),                                //DECT Minigap Fp
	FP_MAC_CON_STAT_REQ(0x03AB, NoParameter),                                        //DECT Minigap Fp
	FP_MAC_CON_STAT_CFM(0x03AC, NoParameter),                                        //DECT Minigap Fp
	FP_MNMM_ACCESS_RIGHTS_REJ_IND(0x03AD, NoParameter),                              //DECT Minigap Fp
	FP_TIMEOUT_FLUSH_LC_BUFFERS_CS(0x03AE, NoParameter),                             //DECT Minigap Fp
	FP_TIMEOUT_FLUSH_LC_BUFFERS_CF(0x03AF, NoParameter),                             //DECT Minigap Fp
	FP_BEARER_ACKNOWLEDGED_RELEASE_IND(0x03B0, NoParameter),                         //DECT Minigap Fp
	FP_DEBUG_FREQ_EXCLUDE_Q_IND(0x03B1, NoParameter),                                //DECT Minigap Fp
	FP_DEBUG_FREQ_EXCLUDE_RSSI_IND(0x03B2, NoParameter),                             //DECT Minigap Fp
	FP_DEBUG_FREQ_INCLUDE_IND(0x03B3, NoParameter),                                  //DECT Minigap Fp
	GET_PROTOCOL_STATE_REQ(0x03B4, NoParameter),                                     //DECT Minigap Fp
	FP_CC_PROTOCOL_STATE_CFM(0x03B5, NoParameter),                                   //DECT Minigap Fp
	FP_MAC_DEBUG_HANDLE_DUMMY_1(0x03B6, NoParameter),                                //DECT Minigap Fp
	FP_MAC_DEBUG_HANDLE_DUMMY_2(0x03B7, NoParameter),                                //DECT Minigap Fp
	FP_MAC_DEBUG_HANDLE_DUMMY_3(0x03B8, NoParameter),                                //DECT Minigap Fp
	FP_MAC_DEBUG_HANDLE_DUMMY_4(0x03B9, NoParameter),                                //DECT Minigap Fp
	FP_BROADCAST_CHANNEL_REQ_1(0x03BA, NoParameter),                                 //DECT Minigap Fp
	FP_MNMM_ULE_AUTHENTICATE_UAK_REQ(0x03BB, NoParameter),                           //DECT Minigap Fp
	FP_MAC_DEBUG_CLOSE_DUMMY(0x03BC, NoParameter),                                   //DECT Minigap Fp
	FP_MAC_REDUCE_POWER_REQ(0x03BD, NoParameter),                                    //DECT Minigap Fp
	FP_BEARER_SET_REDUCED_POWER_REQ(0x03BE, NoParameter),                            //DECT Minigap Fp
	FP_BEARER_COPY_POWER_REQ(0x03BF, NoParameter),                                   //DECT Minigap Fp
	FP_ADVANCED_ASYM_DOWNLINK_REQ(0x03C0, NoParameter),                              //DECT Minigap Fp
	FP_MAC_DEBUG_RSSISCAN_CHANNEL_FOUND(0x03C1, NoParameter),                        //DECT Minigap Fp
	FP_MAC_DEBUG_RSSISCAN_NO_CHANNEL_FOUND(0x03C2, NoParameter),                     //DECT Minigap Fp
	TIMEOUT_FPCCF_WAIT_FOR_FREE_CHANNEL(0x03C3, NoParameter),                        //DECT Minigap Fp
	FP_BEARER_SETUP_FAILED_IND(0x03C4, NoParameter),                                 //DECT Minigap Fp
	FP_BEARER_ESTABLISHED_IND(0x03C5, NoParameter),                                  //DECT Minigap Fp
	FP_MAC_MOD_REQ_BW(0x03C6, NoParameter),                                          //DECT Minigap Fp
	FP_BEARER_BANDWIDTH_REQ(0x03C7, NoParameter),                                    //DECT Minigap Fp
	FP_BEARER_BANDWIDTH_CFM(0x03C8, NoParameter),                                    //DECT Minigap Fp
	FP_BEARER_BANDWIDTH_REJECT_IND(0x03C9, NoParameter),                             //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_SUSPEND_CFM(0x03CA, NoParameter),                      //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_NONE_REQ(0x03CB, NoParameter),                         //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_NONE_CFM(0x03CC, NoParameter),                         //DECT Minigap Fp
	FP_ULP_DLC_SERVICE_CHANGE_FAIL_IND(0x03CD, NoParameter),                         //DECT Minigap Fp
	FP_BMC_DEBUG_DISABLE_ENC(0x03CE, NoParameter),                                   //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_REQ(0x03CF, NoParameter),                                     //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_IND(0x03D0, NoParameter),                                     //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_CFM(0x03D1, NoParameter),                                     //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_CFM_SUCCES(0x03D2, NoParameter),                              //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_CFM_FAILURE(0x03D3, NoParameter),                             //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_RES_SUCCES(0x03D4, NoParameter),                              //DECT Minigap Fp
	FP_MNCC_ULE_MODIFY_RES_FAILURE(0x03D5, NoParameter),                             //DECT Minigap Fp
	FP_MNCC_ULE_IWU_INFO_IND(0x03D6, NoParameter),                                   //DECT Minigap Fp
	FP_MNCC_ULE_IWU_INFO_REQ(0x03D7, NoParameter),                                   //DECT Minigap Fp
	FP_MAC_SET_ENROLL_BIT_SINGLE(0x03D8, NoParameter),                               //DECT Minigap Fp
	FP_MNMM_ENROLL_BIT_CLEARED_IND(0x03D9, NoParameter),                             //DECT Minigap Fp
	FP_ULP_DLC_RESET_REQ(0x03DA, NoParameter),                                       //DECT Minigap Fp
	FP_ULP_DLC_TERMINATE_REQ(0x03DB, NoParameter),                                   //DECT Minigap Fp
	FP_ULP_DLC_POLL_IND(0x03DC, NoParameter),                                        //DECT Minigap Fp
	API_FP_RESET_REQ(0x4000, NoParameter),                                           //ApiFpGeneral
	API_FP_RESET_IND(0x4001, NoParameter),                                           //ApiFpGeneral
	API_FP_GET_FW_VERSION_REQ(0x4002, NoParameter),                                  //ApiFpGeneral
	API_FP_GET_FW_VERSION_CFM(0x4003, NoParameter),                                  //ApiFpGeneral
	API_FP_MM_GET_ID_REQ(0x4004, NoParameter),                                       //ApiFpMm
	API_FP_MM_GET_ID_CFM(0x4005, NoParameter),                                       //ApiFpMm
	API_FP_MM_GET_MODEL_REQ(0x4006, NoParameter),                                    //ApiFpMm
	API_FP_MM_GET_MODEL_CFM(0x4007, NoParameter),                                    //ApiFpMm
	API_FP_MM_SET_ACCESS_CODE_REQ(0x4008, NoParameter),                              //ApiFpMm
	API_FP_MM_SET_ACCESS_CODE_CFM(0x4009, NoParameter),                              //ApiFpMm
	API_FP_MM_GET_ACCESS_CODE_REQ(0x400A, NoParameter),                              //ApiFpMm
	API_FP_MM_GET_ACCESS_CODE_CFM(0x400B, NoParameter),                              //ApiFpMm
	API_FP_SET_CRADLE_STATUS_REQ(0x407F, NoParameter),                               //ApiFpGeneral
	API_FP_CRADLE_DETECT_REQ(0x4080, NoParameter),                                   //ApiFpGeneral
	API_FP_CRADLE_DETECT_IND(0x4081, NoParameter),                                   //ApiFpGeneral
	API_FP_SET_TIME_REQ(0x4082, NoParameter),                                        //ApiFpGeneral
	API_FP_SET_TIME_CFM(0x4083, NoParameter),                                        //ApiFpGeneral
	API_FP_GET_TIME_REQ(0x4084, NoParameter),                                        //ApiFpGeneral
	API_FP_GET_TIME_CFM(0x4085, NoParameter),                                        //ApiFpGeneral
	API_FP_SET_TIME_IND(0x4086, NoParameter),                                        //ApiFpGeneral
	API_FP_SYNC_TIME_REQ(0x4087, NoParameter),                                       //ApiFpGeneral
	API_FP_SET_FEATURES_REQ(0x40A0, NoParameter),                                    //ApiFpGeneral
	API_FP_SET_FEATURES_CFM(0x40A1, NoParameter),                                    //ApiFpGeneral
	API_FP_GET_FEATURES_REQ(0x40A2, NoParameter),                                    //ApiFpGeneral
	API_FP_GET_FEATURES_CFM(0x40A3, NoParameter),                                    //ApiFpGeneral
	API_FP_MM_GET_REGISTRATION_COUNT_REQ(0x4100, NoParameter),                       //ApiFpMm
	API_FP_MM_GET_REGISTRATION_COUNT_CFM(0x4101, NoParameter),                       //ApiFpMm
	API_FP_MM_DELETE_REGISTRATION_REQ(0x4102, NoParameter),                          //ApiFpMm
	API_FP_MM_DELETE_REGISTRATION_CFM(0x4103, NoParameter),                          //ApiFpMm
	API_FP_MM_REGISTRATION_FAILED_IND(0x4104, NoParameter),                          //ApiFpMm
	API_FP_MM_SET_REGISTRATION_MODE_REQ(0x4105, NoParameter),                        //ApiFpMm
	API_FP_MM_SET_REGISTRATION_MODE_CFM(0x4106, NoParameter),                        //ApiFpMm
	API_FP_MM_REGISTRATION_COMPLETE_IND(0x4107, NoParameter),                        //ApiFpMm
	API_FP_MM_HANDSET_PRESENT_IND(0x4108, NoParameter),                              //ApiFpMm
	API_FP_MM_GET_HANDSET_IPUI_REQ(0x4109, NoParameter),                             //ApiFpMm
	API_FP_MM_GET_HANDSET_IPUI_CFM(0x410A, NoParameter),                             //ApiFpMm
	API_FP_MM_STOP_PROTOCOL_REQ(0x410B, NoParameter),                                //ApiFpMm
	API_FP_MM_EXT_HIGHER_LAYER_CAP2_REQ(0x410C, NoParameter),                        //ApiFpMm
	API_FP_MM_START_PROTOCOL_REQ(0x410D, NoParameter),                               //ApiFpMm
	API_FP_MM_HANDSET_DETACH_IND(0x410E, NoParameter),                               //ApiFpMm
	API_FP_MM_HANDSET_DEREGISTERED_IND(0x410F, NoParameter),                         //ApiFpMm
	API_FP_MM_GET_NAME_REQ(0x4110, NoParameter),                                     //ApiFpMm
	API_FP_MM_GET_NAME_CFM(0x4111, NoParameter),                                     //ApiFpMm
	API_FP_MM_SET_NAME_REQ(0x4112, NoParameter),                                     //ApiFpMm
	API_FP_MM_SET_NAME_CFM(0x4113, NoParameter),                                     //ApiFpMm
	API_FP_MM_REGISTRATION_MODE_DISABLED_IND(0x4114, NoParameter),                   //ApiFpMm
	API_FP_MM_SET_FEATURES_REQ(0x4160, NoParameter),                                 //ApiFpMm
	API_FP_MM_SET_FEATURES_CFM(0x4161, NoParameter),                                 //ApiFpMm
	API_FP_MM_GET_FEATURES_REQ(0x4162, NoParameter),                                 //ApiFpMm
	API_FP_MM_GET_FEATURES_CFM(0x4163, NoParameter),                                 //ApiFpMm
	API_FP_MM_GET_FP_CAPABILITIES_REQ(0x4170, NoParameter),                          //ApiFpMm
	API_FP_MM_GET_FP_CAPABILITIES_CFM(0x4171, NoParameter),                          //ApiFpMm
	API_FP_MM_UNITDATA_REQ(0x4180, NoParameter),                                     //ApiFpMm
	API_FP_MM_ALERT_BROADCAST_REQ(0x4182, NoParameter),                              //ApiFpMm
	API_FP_MM_START_PROTOCOL_PCM_SYNC_REQ(0x4183, NoParameter),                      //ApiFpMm
	API_FP_ULE_DATA_IND(0x4185, NoParameter),                                        //ApiFpUle
	API_FP_ULE_DATA_REQ(0x4186, NoParameter),                                        //ApiFpUle
	API_FP_ULE_DTR_IND(0x4187, NoParameter),                                         //ApiFpUle
	API_FP_ULE_INIT_REQ(0x4188, NoParameter),                                        //ApiFpUle
	API_FP_ULE_INIT_CFM(0x4189, NoParameter),                                        //ApiFpUle
	API_FP_ULE_TESTCMD_REQ(0x418A, NoParameter),                                     //ApiFpUle
	API_FP_ULE_DATA_CFM(0x418B, NoParameter),                                        //ApiFpUle
	API_FP_ULE_GET_REGISTRATION_COUNT_REQ(0x418C, NoParameter),                      //ApiFpUle
	API_FP_ULE_GET_REGISTRATION_COUNT_CFM(0x418D, NoParameter),                      //ApiFpUle
	API_FP_ULE_GET_DEVICE_IPUI_REQ(0x418E, NoParameter),                             //ApiFpUle
	API_FP_ULE_GET_DEVICE_IPUI_CFM(0x418F, NoParameter),                             //ApiFpUle
	API_FP_ULE_ABORT_DATA_REQ(0x4190, NoParameter),                                  //ApiFpUle
	API_FP_ULE_ABORT_DATA_CFM(0x4191, NoParameter),                                  //ApiFpUle
	API_FP_ULE_DELETE_REGISTRATION_REQ(0x4192, NoParameter),                         //ApiFpUle
	API_FP_ULE_DELETE_REGISTRATION_CFM(0x4193, NoParameter),                         //ApiFpUle
	API_FP_ULE_SET_PVC_LEGACY_MODE_REQ(0x419E, NoParameter),                         //ApiFpUle
	API_FP_ULE_SET_FEATURES_REQ(0x41A0, NoParameter),                                //ApiFpUle
	API_FP_ULE_SET_FEATURES_CFM(0x41A1, NoParameter),                                //ApiFpUle
	API_FP_ULE_PVC_CONFIG_RES(0x41A2, NoParameter),                                  //ApiFpUle
	API_FP_ULE_PVC_CONFIG_CFM(0x41A3, NoParameter),                                  //ApiFpUle
	API_FP_ULE_PVC_CONFIG_REJ(0x41A4, NoParameter),                                  //ApiFpUle
	API_FP_ULE_PVC_CONFIG_IND(0x41A5, NoParameter),                                  //ApiFpUle
	API_FP_ULE_PVC_PENDING_IND(0x41A6, NoParameter),                                 //ApiFpUle
	API_FP_ULE_PVC_PENDING_RES(0x41A7, NoParameter),                                 //ApiFpUle
	API_FP_ULE_PVC_IWU_DATA_REQ(0x41A8, NoParameter),                                //ApiFpUle
	API_FP_ULE_PVC_IWU_DATA_IND(0x41A9, NoParameter),                                //ApiFpUle
	API_FP_ULE_GET_FEATURES_REQ(0x41AA, NoParameter),                                //ApiFpUle
	API_FP_ULE_GET_FEATURES_CFM(0x41AB, NoParameter),                                //ApiFpUle
	API_FP_ULE_DATA_POLL_REQ(0x41AC, NoParameter),                                   //ApiFpUle
	API_FP_ULE_DATA_POLL_CFM(0x41AD, NoParameter),                                   //ApiFpUle
	API_FP_INIT_PCM_REQ(0x4200, NoParameter),                                        //ApiFpAudio
	API_FP_INIT_PCM_CFM(0x4201, NoParameter),                                        //ApiFpAudio
	API_FP_SET_PCM_LOOPBACK_REQ(0x4204, NoParameter),                                //ApiFpAudio
	API_FP_SET_PCM_LOOPBACK_CFM(0x4205, NoParameter),                                //ApiFpAudio
	API_FP_INIT_USB_REQ(0x4206, NoParameter),                                        //ApiFpAudio
	API_FP_INIT_USB_CFM(0x4207, NoParameter),                                        //ApiFpAudio
	API_FP_SET_AUDIO_FORMAT_REQ(0x4210, NoParameter),                                //ApiFpAudio
	API_FP_SET_AUDIO_FORMAT_CFM(0x4211, NoParameter),                                //ApiFpAudio
	API_FP_AUDIO_MUTE_REQ(0x4212, NoParameter),                                      //ApiFpAudio
	API_FP_AUDIO_MUTE_CFM(0x4213, NoParameter),                                      //ApiFpAudio
	API_FP_AUDIO_UNMUTE_REQ(0x4214, NoParameter),                                    //ApiFpAudio
	API_FP_AUDIO_UNMUTE_CFM(0x4215, NoParameter),                                    //ApiFpAudio
	API_FP_AUDIO_INTERNAL_CODEC_SETUP_REQ(0x4216, NoParameter),                      //ApiFpAudio
	API_FP_AUDIO_INTERNAL_CODEC_SETUP_CFM(0x4217, NoParameter),                      //ApiFpAudio
	API_FP_AUDIO_SET_HW_SRC_REQ(0x4218, NoParameter),                                //ApiFpAudio
	API_FP_AUDIO_SET_HW_SRC_CFM(0x4219, NoParameter),                                //ApiFpAudio
	API_FP_AUDIO_SET_CODEC_GAIN_REQ(0x421A, NoParameter),                            //ApiFpAudio
	API_FP_AUDIO_SET_CODEC_GAIN_CFM(0x421B, NoParameter),                            //ApiFpAudio
	API_FP_AUDIO_ENABLE_EC_REQ(0x421C, NoParameter),                                 //ApiFpAudio
	API_FP_AUDIO_ENABLE_EC_CFM(0x421D, NoParameter),                                 //ApiFpAudio
	API_FP_AUDIO_SET_FEATURES_REQ(0x4220, NoParameter),                              //ApiFpAudio
	API_FP_AUDIO_SET_FEATURES_CFM(0x4221, NoParameter),                              //ApiFpAudio
	API_FP_AUDIO_GET_FEATURES_REQ(0x4223, NoParameter),                              //ApiFpAudio
	API_FP_AUDIO_GET_FEATURES_CFM(0x4224, NoParameter),                              //ApiFpAudio
	API_FP_CC_SETUP_IND(0x4400, NoParameter),                                        //ApiFpCc
	API_FP_CC_SETUP_RES(0x4401, NoParameter),                                        //ApiFpCc
	API_FP_CC_SETUP_REQ(0x4402, NoParameter),                                        //ApiFpCc
	API_FP_CC_SETUP_CFM(0x4403, NoParameter),                                        //ApiFpCc
	API_FP_CC_ALERT_IND(0x4404, NoParameter),                                        //ApiFpCc
	API_FP_CC_ALERT_REQ(0x4405, NoParameter),                                        //ApiFpCc
	API_FP_CC_ALERT_CFM(0x4406, NoParameter),                                        //ApiFpCc
	API_FP_CC_CONNECT_IND(0x4407, NoParameter),                                      //ApiFpCc
	API_FP_CC_CONNECT_RES(0x4408, NoParameter),                                      //ApiFpCc
	API_FP_CC_CONNECT_REQ(0x4409, NoParameter),                                      //ApiFpCc
	API_FP_CC_CONNECT_CFM(0x440A, NoParameter),                                      //ApiFpCc
	API_FP_CC_RELEASE_IND(0x440F, NoParameter),                                      //ApiFpCc
	API_FP_CC_RELEASE_RES(0x4410, NoParameter),                                      //ApiFpCc
	API_FP_CC_RELEASE_REQ(0x4411, NoParameter),                                      //ApiFpCc
	API_FP_CC_RELEASE_CFM(0x4412, NoParameter),                                      //ApiFpCc
	API_FP_CC_REJECT_IND(0x4413, NoParameter),                                       //ApiFpCc
	API_FP_CC_CALL_PROC_REQ(0x4415, NoParameter),                                    //ApiFpCc
	API_FP_CC_CALL_PROC_CFM(0x4416, NoParameter),                                    //ApiFpCc
	API_FP_CC_MODIFY_CODEC_REQ(0x441D, NoParameter),                                 //ApiFpCc
	API_FP_CC_MODIFY_CODEC_CFM(0x441E, NoParameter),                                 //ApiFpCc
	API_FP_CC_MODIFY_CODEC_IND(0x441F, NoParameter),                                 //ApiFpCc
	API_FP_CC_MODIFY_CODEC_RES(0x4420, NoParameter),                                 //ApiFpCc
	API_FP_CC_IWU_INFO_CODEC_IND(0x4421, NoParameter),                               //ApiFpCc
	API_FP_CC_SETUP_ACK_REQ(0x4423, NoParameter),                                    //ApiFpCc
	API_FP_CC_SETUP_ACK_CFM(0x4424, NoParameter),                                    //ApiFpCc
	API_FP_CC_SYSTEM_CALL_ID_REQ(0x4427, NoParameter),                               //ApiFpCc
	API_FP_CC_SYSTEM_CALL_ID_CFM(0x4428, NoParameter),                               //ApiFpCc
	API_FP_CC_RECONNECT_AUDIO_REQ(0x442C, NoParameter),                              //ApiFpCc
	API_FP_CC_INFO_IND(0x442D, NoParameter),                                         //ApiFpCc
	API_FP_CC_INFO_REQ(0x442E, NoParameter),                                         //ApiFpCc
	API_FP_CC_DISCONNECT_AUDIO_REQ(0x442F, NoParameter),                             //ApiFpCc
	API_FP_CC_SELECTED_ADPCM_IND(0x4430, NoParameter),                               //ApiFpCc
	API_FP_CC_SET_FEATURES_REQ(0x4450, NoParameter),                                 //ApiFpCc
	API_FP_CC_SET_FEATURES_CFM(0x4451, NoParameter),                                 //ApiFpCc
	API_FP_CC_GET_FEATURES_REQ(0x4452, NoParameter),                                 //ApiFpCc
	API_FP_CC_GET_FEATURES_CFM(0x4453, NoParameter),                                 //ApiFpCc
	API_FP_MAC_NO_EMISSION_MODE_ENABLE_REQ(0x4600, NoParameter),                     //ApiFpNoEmission
	API_FP_MAC_NO_EMISSION_MODE_DISABLE_REQ(0x4601, NoParameter),                    //ApiFpNoEmission
	API_FP_MAC_NO_EMISSION_MODE_STOP_REQ(0x4602, NoParameter),                       //ApiFpNoEmission
	API_FP_MAC_NO_EMISSION_MODE_PENDING_IND(0x4603, NoParameter),                    //ApiFpNoEmission
	API_FP_MAC_NO_EMISSION_MODE_ACTIVE_IND(0x4604, NoParameter),                     //ApiFpNoEmission
	API_FP_MAC_NO_EMISSION_MODE_STOP_IND(0x4605, NoParameter),                       //ApiFpNoEmission
	API_FP_MAC_NO_EMISSION_MODE_SET_PREF_CARRIER_REQ(0x4606, NoParameter),           //ApiFpNoEmission
	API_FP_LINUX_INIT_REQ(0x4700, NoParameter),                                      //ApiFpLinux
	API_FP_LINUX_INIT_DEFAULT_REQ(0x4701, NoParameter),                              //ApiFpLinux
	API_FP_LINUX_INIT_CFM(0x4702, NoParameter),                                      //ApiFpLinux
	API_FP_LINUX_NVS_UPDATE_IND(0x4703, NoParameter),                                //ApiFpLinux
	API_FP_LINUX_INTERNAL_ERROR(0x4704, NoParameter),                                //ApiFpLinux
	API_FP_INTERNAL_SWAP_API_TO_EAI(0x470F, NoParameter),                            //Natalie V3 CVM
	API_LINUX_INIT_GET_SYSTEM_INFO_REQ(0x4710, NoParameter),                         //ApiLinux
	API_LINUX_INIT_GET_SYSTEM_INFO_CFM(0x4711, NoParameter),                         //ApiLinux
	API_LINUX_INIT_REQ(0x4712, NoParameter),                                         //ApiLinux
	API_LINUX_INIT_CFM(0x4713, NoParameter),                                         //ApiLinux
	API_LINUX_NVS_UPDATE_IND(0x4714, NoParameter),                                   //ApiLinux
	API_LINUX_INTERNAL_ERROR_IND(0x4715, NoParameter),                               //ApiLinux
	API_SCL_GET_MODE_REQ(0x4740, NoParameter),                                       //ApiScl
	API_SCL_GET_MODE_CFM(0x4741, NoParameter),                                       //ApiScl
	API_SCL_SET_MODE_REQ(0x4742, NoParameter),                                       //ApiScl
	API_SCL_SET_MODE_CFM(0x4743, NoParameter),                                       //ApiScl
	API_SCL_GET_URL_REQ(0x4744, NoParameter),                                        //ApiScl
	API_SCL_GET_URL_CFM(0x4745, NoParameter),                                        //ApiScl
	API_SCL_SET_URL_REQ(0x4746, NoParameter),                                        //ApiScl
	API_SCL_SET_URL_CFM(0x4747, NoParameter),                                        //ApiScl
	API_SCL_OPERATION_REQ(0x4748, NoParameter),                                      //ApiScl
	API_SCL_OPERATION_CFM(0x4749, NoParameter),                                      //ApiScl
	API_SCL_STATUS_IND(0x474A, NoParameter),                                         //ApiScl
	API_SCL_VERSIONS_IND(0x474B, NoParameter),                                       //ApiScl
	API_SCL_GET_GUID_REQ(0x474C, NoParameter),                                       //ApiScl
	API_SCL_GET_GUID_CFM(0x474D, NoParameter),                                       //ApiScl
	API_SCL_SET_GUID_REQ(0x474E, NoParameter),                                       //ApiScl
	API_SCL_SET_GUID_CFM(0x474F, NoParameter),                                       //ApiScl
	API_FP_LLME_GET_RSSI_REQ(0x47B0, NoParameter),                                   //ApiFpLlme
	API_FP_LLME_GET_RSSI_CFM(0x47B1, NoParameter),                                   //ApiFpLlme
	API_FP_LLME_GET_PERFORMANCE_TABLE_REQ(0x47B2, NoParameter),                      //ApiFpLlme
	API_FP_LLME_GET_PERFORMANCE_TABLE_CFM(0x47B3, NoParameter),                      //ApiFpLlme
	API_FP_LLME_SET_FEATURES_REQ(0x47B4, NoParameter),                               //ApiFpLlme
	API_FP_LLME_SET_FEATURES_CFM(0x47B5, NoParameter),                               //ApiFpLlme
	API_FP_LLME_GET_FEATURES_REQ(0x47B6, NoParameter),                               //ApiFpLlme
	API_FP_LLME_GET_FEATURES_CFM(0x47B7, NoParameter),                               //ApiFpLlme
	API_ISP_IDENTIFY_REQ(0x47D0, NoParameter),                                       //ApiIsp
	API_ISP_IDENTIFY_CFM(0x47D1, NoParameter),                                       //ApiIsp
	API_ISP_ERASE_REQ(0x47D2, NoParameter),                                          //ApiIsp
	API_ISP_ERASE_CFM(0x47D3, NoParameter),                                          //ApiIsp
	API_ISP_WRITE_REQ(0x47D4, NoParameter),                                          //ApiIsp
	API_ISP_WRITE_CFM(0x47D5, NoParameter),                                          //ApiIsp
	API_ISP_CRC_REQ(0x47D6, NoParameter),                                            //ApiIsp
	API_ISP_CRC_CFM(0x47D7, NoParameter),                                            //ApiIsp
	API_ISP_EMPTY_REQ(0x47D8, NoParameter),                                          //ApiIsp
	API_ISP_EMPTY_CFM(0x47D9, NoParameter),                                          //ApiIsp
	API_ISP_MD5_REQ(0x47DA, NoParameter),                                            //ApiIsp
	API_ISP_MD5_CFM(0x47DB, NoParameter),                                            //ApiIsp
	API_ISP_WRPTR_REQ(0x47DC, NoParameter),                                          //ApiIsp
	API_LDS_OPEN_REQ(0x4800, NoParameter),                                           //ApiLds
	API_LDS_OPEN_CFM(0x4801, NoParameter),                                           //ApiLds
	API_LDS_OPEN_IND(0x4802, NoParameter),                                           //ApiLds
	API_LDS_OPEN_RES(0x4803, NoParameter),                                           //ApiLds
	API_LDS_CLOSE_REQ(0x4804, NoParameter),                                          //ApiLds
	API_LDS_CLOSE_CFM(0x4805, NoParameter),                                          //ApiLds
	API_LDS_CLOSE_IND(0x4806, NoParameter),                                          //ApiLds
	API_LDS_REJECT_REQ(0x4807, NoParameter),                                         //ApiLds
	API_LDS_REJECT_IND(0x4808, NoParameter),                                         //ApiLds
	API_LDS_TX_READY_IND(0x4820, NoParameter),                                       //ApiLds
	API_LDS_TX_DATA_REQ(0x4821, NoParameter),                                        //ApiLds
	API_LDS_TX_DATA_CFM(0x4822, NoParameter),                                        //ApiLds
	API_LDS_RX_DATA_IND(0x4823, NoParameter),                                        //ApiLds
	API_LDS_SET_FEATURES_REQ(0x4830, NoParameter),                                   //ApiLds
	API_LDS_SET_FEATURES_CFM(0x4831, NoParameter),                                   //ApiLds
	API_LDS_GET_FEATURES_REQ(0x4832, NoParameter),                                   //ApiLds
	API_LDS_GET_FEATURES_CFM(0x4833, NoParameter),                                   //ApiLds
	API_SUOTA_SW_VERSION_INFO_REQ(0x4880, NoParameter),                              //ApiSuota
	API_SUOTA_SW_VERSION_INFO_IND(0x4881, NoParameter),                              //ApiSuota
	API_SUOTA_NEW_SW_AVAILABLE_REQ(0x4882, NoParameter),                             //ApiSuota
	API_SUOTA_NEW_SW_AVAILABLE_IND(0x4883, NoParameter),                             //ApiSuota
	API_SUOTA_NEGATIVE_ACK_REQ(0x4884, NoParameter),                                 //ApiSuota
	API_SUOTA_NEGATIVE_ACK_IND(0x4885, NoParameter),                                 //ApiSuota
	API_SUOTA_SET_FEATURES_REQ(0x4890, NoParameter),                                 //ApiSuota
	API_SUOTA_SET_FEATURES_CFM(0x4891, NoParameter),                                 //ApiSuota
	API_SUOTA_GET_FEATURES_REQ(0x4892, NoParameter),                                 //ApiSuota
	API_SUOTA_GET_FEATURES_CFM(0x4893, NoParameter),                                 //ApiSuota
	API_FP_WIFI_EXCLUDE_REQ(0x4900, NoParameter),                                    //ApiFpWExcl
	API_FP_WIFI_EXCLUDE_CFM(0x4901, NoParameter),                                    //ApiFpWExcl
	API_FWU_ENABLE_REQ(0x4F00, NoParameter),                                         //ApiFwu
	API_FWU_ENABLE_CFM(0x4F01, NoParameter),                                         //ApiFwu
	API_FWU_DEVICE_NOTIFY_IND(0x4F02, NoParameter),                                  //ApiFwu
	API_FWU_UPDATE_REQ(0x4F03, NoParameter),                                         //ApiFwu
	API_FWU_UPDATE_CFM(0x4F04, NoParameter),                                         //ApiFwu
	API_FWU_UPDATE_IND(0x4F05, NoParameter),                                         //ApiFwu
	API_FWU_UPDATE_RES(0x4F06, NoParameter),                                         //ApiFwu
	API_FWU_GET_BLOCK_IND(0x4F07, NoParameter),                                      //ApiFwu
	API_FWU_GET_BLOCK_RES(0x4F08, NoParameter),                                      //ApiFwu
	API_FWU_GET_CRC_IND(0x4F09, NoParameter),                                        //ApiFwu
	API_FWU_GET_CRC_RES(0x4F0A, NoParameter),                                        //ApiFwu
	API_FWU_COMPLETE_IND(0x4F0B, NoParameter),                                       //ApiFwu
	API_FWU_STATUS_IND(0x4F0C, NoParameter),                                         //ApiFwu
	API_FWU_MULTI_CRC_IND(0x4F0D, NoParameter),                                      //ApiFwu
	API_FWU_MULTI_CRC_RES(0x4F0E, NoParameter),                                      //ApiFwu
	API_FWU_SET_FEATURES_REQ(0x4F10, NoParameter),                                   //ApiFwu
	API_FWU_SET_FEATURES_CFM(0x4F11, NoParameter),                                   //ApiFwu
	API_FWU_GET_FEATURES_REQ(0x4F12, NoParameter),                                   //ApiFwu
	API_FWU_GET_FEATURES_CFM(0x4F13, NoParameter),                                   //ApiFwu
	API_PROD_TEST_REQ(0x4FFE, NoParameter),                                          //ApiProdTest
	API_PROD_TEST_CFM(0x4FFF, NoParameter),                                          //ApiProdTest
	API_AUDIO_STOP_TONE_REQ(0x5307, NoParameter),                                    //ApiFpProjectAudio
	API_AUDIO_STOP_MIDI_REQ(0x530C, NoParameter),                                    //ApiFpProjectAudio
	API_AUDIO_MIDI_END_IND(0x530D, NoParameter),                                     //ApiFpProjectAudio
	API_AUDIO_TONE_END_IND(0x5313, NoParameter),                                     //ApiFpProjectAudio
	API_AUDIO_START_TONE_CFM(0x5314, NoParameter),                                   //ApiFpProjectAudio
	API_AUDIO_STOP_TONE_CFM(0x5315, NoParameter),                                    //ApiFpProjectAudio
	API_AUDIO_START_MIDI_CFM(0x5316, NoParameter),                                   //ApiFpProjectAudio
	API_AUDIO_STOP_MIDI_CFM(0x5317, NoParameter),                                    //ApiFpProjectAudio
	API_AUDIO_START_TONE_EXT_REQ(0x5328, NoParameter),                               //ApiFpProjectAudio
	API_AUDIO_START_MIDI_EXT_REQ(0x5329, NoParameter),                               //ApiFpProjectAudio
	API_LAS_START_SESSION_REQ(0x5500, NoParameter),                                  //ApiLas
	API_LAS_START_SESSION_CFM(0x5501, NoParameter),                                  //ApiLas
	API_LAS_END_SESSION_REQ(0x5502, NoParameter),                                    //ApiLas
	API_LAS_END_SESSION_CFM(0x5503, NoParameter),                                    //ApiLas
	API_LAS_QUERY_SUPPORTED_ENTRY_FIELDS_REQ(0x5504, NoParameter),                   //ApiLas
	API_LAS_QUERY_SUPPORTED_ENTRY_FIELDS_CFM(0x5505, NoParameter),                   //ApiLas
	API_LAS_READ_ENTRIES_REQ(0x5506, NoParameter),                                   //ApiLas
	API_LAS_READ_ENTRIES_CFM(0x5507, NoParameter),                                   //ApiLas
	API_LAS_EDIT_ENTRY_REQ(0x5508, NoParameter),                                     //ApiLas
	API_LAS_EDIT_ENTRY_CFM(0x5509, NoParameter),                                     //ApiLas
	API_LAS_SAVE_ENTRY_REQ(0x550A, NoParameter),                                     //ApiLas
	API_LAS_SAVE_ENTRY_CFM(0x550B, NoParameter),                                     //ApiLas
	API_LAS_DELETE_ENTRY_REQ(0x550C, NoParameter),                                   //ApiLas
	API_LAS_DELETE_ENTRY_CFM(0x550D, NoParameter),                                   //ApiLas
	API_LAS_DELETE_LIST_REQ(0x550E, NoParameter),                                    //ApiLas
	API_LAS_DELETE_LIST_CFM(0x550F, NoParameter),                                    //ApiLas
	API_LAS_SEARCH_ENTRIES_REQ(0x5510, NoParameter),                                 //ApiLas
	API_LAS_SEARCH_ENTRIES_CFM(0x5511, NoParameter),                                 //ApiLas
	API_LAS_NEGATIVE_ACKNOWLEDGE_IND(0x5512, NoParameter),                           //ApiLas
	API_LAS_DATA_PACKET_IND(0x5513, NoParameter),                                    //ApiLas
	API_LAS_BASE_RESET_IND(0x5580, NoParameter),                                     //ApiLas
	API_LAS_STATUS_IND(0x5581, NoParameter),                                         //ApiLas
	API_LAS_GET_FIELD_PROTECTION_REQ(0x5582, NoParameter),                           //ApiLas
	API_LAS_GET_FIELD_PROTECTION_CFM(0x5583, NoParameter),                           //ApiLas
	API_LAS_SET_FIELD_PROTECTION_REQ(0x5584, NoParameter),                           //ApiLas
	API_LAS_SET_FIELD_PROTECTION_CFM(0x5585, NoParameter),                           //ApiLas
	API_LAS_GET_LOCKED_ENTRIES_LIST_REQ(0x5586, NoParameter),                        //ApiLas
	API_LAS_GET_LOCKED_ENTRIES_LIST_CFM(0x5587, NoParameter),                        //ApiLas
	API_LAS_SET_FIELD_DEFAULT_REQ(0x5588, NoParameter),                              //ApiLas
	API_LAS_SET_FIELD_DEFAULT_CFM(0x5589, NoParameter),                              //ApiLas
	API_LAS_DB_CLEAR_REQ(0x55C0, NoParameter),                                       //ApiLas
	API_LAS_DB_CLEAR_CFM(0x55C1, NoParameter),                                       //ApiLas
	API_LAS_DB_QUERY_SUPPORTED_LISTS_IND(0x5600, NoParameter),                       //ApiLasDb
	API_LAS_DB_QUERY_SUPPORTED_LISTS_RES(0x5601, NoParameter),                       //ApiLasDb
	API_LAS_DB_QUERY_SUPPORTED_ENTRY_FIELDS_IND(0x5602, NoParameter),                //ApiLasDb
	API_LAS_DB_QUERY_SUPPORTED_ENTRY_FIELDS_RES(0x5603, NoParameter),                //ApiLasDb
	API_LAS_DB_LIST_SORTING_REQUEST_IND(0x5604, NoParameter),                        //ApiLasDb
	API_LAS_DB_LIST_SORTING_REQUEST_RES(0x5605, NoParameter),                        //ApiLasDb
	API_LAS_DB_LIST_SORTING_RELEASE_IND(0x5606, NoParameter),                        //ApiLasDb
	API_LAS_DB_LIST_SORTING_RELEASE_RES(0x5607, NoParameter),                        //ApiLasDb
	API_LAS_DB_TOTAL_ENTRIES_IND(0x5608, NoParameter),                               //ApiLasDb
	API_LAS_DB_TOTAL_ENTRIES_RES(0x5609, NoParameter),                               //ApiLasDb
	API_LAS_DB_READ_IND(0x560A, NoParameter),                                        //ApiLasDb
	API_LAS_DB_READ_RES(0x560B, NoParameter),                                        //ApiLasDb
	API_LAS_DB_EDIT_IND(0x560C, NoParameter),                                        //ApiLasDb
	API_LAS_DB_EDIT_RES(0x560D, NoParameter),                                        //ApiLasDb
	API_LAS_DB_SAVE_IND(0x560E, NoParameter),                                        //ApiLasDb
	API_LAS_DB_SAVE_RES(0x560F, NoParameter),                                        //ApiLasDb
	API_LAS_DB_SEARCH_IND(0x5610, NoParameter),                                      //ApiLasDb
	API_LAS_DB_SEARCH_RES(0x5611, NoParameter),                                      //ApiLasDb
	API_LAS_DB_DELETE_IND(0x5612, NoParameter),                                      //ApiLasDb
	API_LAS_DB_DELETE_RES(0x5613, NoParameter),                                      //ApiLasDb
	API_LAS_DB_DELETE_LIST_IND(0x5614, NoParameter),                                 //ApiLasDb
	API_LAS_DB_DELETE_LIST_RES(0x5615, NoParameter),                                 //ApiLasDb
	API_LAS_DB_NEGATIVE_ACKNOWLEDGE_REQ(0x5616, NoParameter),                        //ApiLasDb
	API_LAS_DB_NEGATIVE_ACKNOWLEDGE_IND(0x5617, NoParameter),                        //ApiLasDb
	API_LAS_DB_DATA_PACKET_IND(0x5618, NoParameter),                                 //ApiLasDb
	API_LAS_DB_DATA_PACKET_RES(0x5619, NoParameter),                                 //ApiLasDb
	API_LAS_DB_DATA_PACKET_REQ(0x561A, NoParameter),                                 //ApiLasDb
	API_LAS_DB_DATA_PACKET_CFM(0x561B, NoParameter),                                 //ApiLasDb
	API_LAS_DB_RESET_IND(0x5630, NoParameter),                                       //ApiLasDb
	API_LAS_DB_RESET_RES(0x5631, NoParameter),                                       //ApiLasDb
	API_LAS_DB_GET_FIELD_PROTECTION_IND(0x5632, NoParameter),                        //ApiLasDb
	API_LAS_DB_GET_FIELD_PROTECTION_RES(0x5633, NoParameter),                        //ApiLasDb
	API_LAS_DB_SET_FIELD_PROTECTION_IND(0x5634, NoParameter),                        //ApiLasDb
	API_LAS_DB_SET_FIELD_PROTECTION_RES(0x5635, NoParameter),                        //ApiLasDb
	API_LAS_DB_SET_FIELD_DEFAULT_IND(0x5636, NoParameter),                           //ApiLasDb
	API_LAS_DB_SET_FIELD_DEFAULT_RES(0x5637, NoParameter),                           //ApiLasDb
	API_LAS_DB_SET_ACTIVE_DB_REQ(0x5650, NoParameter),                               //ApiLasDb
	API_LAS_DB_SET_ACTIVE_DB_CFM(0x5651, NoParameter),                               //ApiLasDb
	API_LAS_DB_GET_ACTIVE_DB_REQ(0x5652, NoParameter),                               //ApiLasDb
	API_LAS_DB_GET_ACTIVE_DB_CFM(0x5653, NoParameter),                               //ApiLasDb
	API_LAS_DB_CLEAR_DB_IND(0x56C0, NoParameter),                                    //ApiLasDb
	API_LAS_DB_CLEAR_DB_RES(0x56C1, NoParameter),                                    //ApiLasDb
	API_FP_GENEVENOT_EVENT_REQ(0x5700, NoParameter),                                 //ApiGenEveNot
	API_FP_GENEVENOT_EVENT_IND(0x5701, NoParameter),                                 //ApiGenEveNot
	API_FP_GENEVENOT_PP_EVENT_IND(0x5702, NoParameter),                              //ApiGenEveNot
	API_PP_GENEVENOT_EVENT_REQ(0x5703, NoParameter),                                 //ApiGenEveNot
	API_PP_GENEVENOT_EVENT_IND(0x5704, NoParameter),                                 //ApiGenEveNot
	API_FP_GENEVENOT_FEATURES_REQ(0x5708, NoParameter),                              //ApiGenEveNot
	API_FP_GENEVENOT_FEATURES_CFM(0x5709, NoParameter),                              //ApiGenEveNot
	API_IMAGE_INFO_REQ(0x5800, NoParameter),                                         //ApiImageControl
	API_IMAGE_INFO_CFM(0x5801, NoParameter),                                         //ApiImageControl
	API_IMAGE_ACTIVATE_REQ(0x5802, NoParameter),                                     //ApiImageControl
	API_IMAGE_ACTIVATE_CFM(0x5803, NoParameter),                                     //ApiImageControl
	API_HAL_DEVICE_CONTROL_REQ(0x5900, NoParameter),                                 //ApiHal
	API_HAL_DEVICE_CONTROL_CFM(0x5901, NoParameter),                                 //ApiHal
	API_HAL_LED_REQ(0x5902, api_hal::LedReq),                                        //ApiHal
	API_HAL_LED_CFM(0x5903, NoParameter),                                            //ApiHal
	API_HAL_READ_REQ(0x5904, NoParameter),                                           //ApiHal
	API_HAL_READ_CFM(0x5905, NoParameter),                                           //ApiHal
	API_HAL_WRITE_REQ(0x5906, NoParameter),                                          //ApiHal
	API_HAL_WRITE_CFM(0x5907, NoParameter),                                          //ApiHal
	API_HAL_GPIO_FN_REGISTER_REQ(0x5910, NoParameter),                               //ApiHal
	API_HAL_GPIO_FN_REGISTER_CFM(0x5911, NoParameter),                               //ApiHal
	API_HAL_SET_GPIO_PORT_PIN_MODE_REQ(0x5912, NoParameter),                         //ApiHal
	API_HAL_SET_GPIO_PORT_PIN_MODE_CFM(0x5913, NoParameter),                         //ApiHal
	API_HAL_SET_GPIO_PORT_REQ(0x5914, NoParameter),                                  //ApiHal
	API_HAL_SET_GPIO_PORT_CFM(0x5915, NoParameter),                                  //ApiHal
	API_HAL_RESET_GPIO_PORT_REQ(0x5916, NoParameter),                                //ApiHal
	API_HAL_RESET_GPIO_PORT_CFM(0x5917, NoParameter),                                //ApiHal
	API_HAL_GET_GPIO_PORT_REQ(0x5918, NoParameter),                                  //ApiHal
	API_HAL_GET_GPIO_PORT_CFM(0x5919, NoParameter),                                  //ApiHal
	API_CLSS_REQ(0x5C40, NoParameter),                                               //ApiClss
	API_CLSS_IND(0x5C41, NoParameter),                                               //ApiClss
	API_CLSS_SET_IWU_TO_IWU_REQ(0x5C42, NoParameter),                                //ApiClss
	API_CLSS_SET_IWU_TO_IWU_CFM(0x5C43, NoParameter),                                //ApiClss
	API_CLSS_SET_FEATURES_REQ(0x5C48, NoParameter),                                  //ApiClss
	API_CLSS_SET_FEATURES_CFM(0x5C49, NoParameter),                                  //ApiClss
	API_CLSS_GET_FEATURES_REQ(0x5C4A, NoParameter),                                  //ApiClss
	API_CLSS_GET_FEATURES_CFM(0x5C4B, NoParameter),                                  //ApiClss
	RTX_EAP_TRACE_START_REQ(0xF000, NoParameter),                                    //RtxEai
	RTX_EAP_TRACE_START_CFM(0xF001, NoParameter),                                    //RtxEai
	RTX_EAP_TRACE_STOP_REQ(0xF002, NoParameter),                                     //RtxEai
	RTX_EAP_TRACE_STOP_CFM(0xF003, NoParameter),                                     //RtxEai
	RTX_EAP_TRACE_DOWNLOAD_REQ(0xF004, NoParameter),                                 //RtxEai
	RTX_EAP_TRACE_DOWNLOAD_CFM(0xF005, NoParameter),                                 //RtxEai
	RTX_EAP_TRACE_DOWNLOAD_START_REQ(0xF006, NoParameter),                           //RtxEai
	RTX_EAP_TRACE_START_TIMER_IND(0xF007, NoParameter),                              //RtxEai
	RTX_EAP_TRACE_STOP_TIMER_IND(0xF008, NoParameter),                               //RtxEai
	RTX_EAP_TRACE_OUTPUT_MAIL_IND(0xF009, NoParameter),                              //RtxEai
	RTX_EAP_TRACE_INPUT_MAIL_IND(0xF00A, NoParameter),                               //RtxEai
	RTX_EAP_TRACE_COMMENT_IND(0xF00B, NoParameter),                                  //RtxEai
	RTX_EAP_TRACE_STOP_IND(0xF00C, NoParameter),                                     //RtxEai
	RTX_EAP_TRACE_PDU_IND(0xF00D, NoParameter),                                      //RtxEai
	RTX_EAP_TRACE_FUNCTION_IND(0xF00E, NoParameter),                                 //RtxEai
	RTX_EAP_TRACE_DATA_IND(0xF00F, NoParameter),                                     //RtxEai
	RTX_EAP_TRACE_IND(0xF010, NoParameter),                                          //RtxEai
	RTX_EAP_RUNTIME_ERROR_IND(0xF011, NoParameter),                                  //RtxEai
	RTX_EAP_ASSERT_IND(0xF012, NoParameter),                                         //RtxEai
	RTX_EAP_TRACE_TIME_INPUT_MAIL_IND(0xF013, NoParameter),                          //RtxEai
	RTX_EAP_TRACE_TIME_OUTPUT_MAIL_IND(0xF014, NoParameter),                         //RtxEai
	RTX_EAP_RESERVED_MAIL_2(0xF015, NoParameter),                                    //RtxEai
	RTX_EAP_TRACE_TIME_START_TIMER_IND(0xF016, NoParameter),                         //RtxEai
	RTX_EAP_TRACE_TIME_STOP_TIMER_IND(0xF017, NoParameter),                          //RtxEai
	RTX_EAP_RESERVED_MAIL_1(0xF018, NoParameter),                                    //RtxEai
	RTX_EAP_TARGET_TIME_INFO_IND(0xF019, NoParameter),                               //RtxEai
	RTX_EAP_TRACE_COMMENT2_IND(0xF01A, NoParameter),                                 //RtxEai
	RTX_EAP_TRACE_TIME_COMMENT2_IND(0xF01B, NoParameter),                            //RtxEai
	RTX_EAP_AFIELD_START_REQ(0xF020, NoParameter),                                   //RtxEai
	RTX_EAP_AFIELD_START_CFM(0xF021, NoParameter),                                   //RtxEai
	RTX_EAP_AFIELD_STOP_REQ(0xF022, NoParameter),                                    //RtxEai
	RTX_EAP_AFIELD_STOP_CFM(0xF023, NoParameter),                                    //RtxEai
	RTX_EAP_AFIELD_DATA_FP_RX_IND(0xF024, NoParameter),                              //RtxEai
	RTX_EAP_AFIELD_DATA_FP_TX_IND(0xF025, NoParameter),                              //RtxEai
	RTX_EAP_AFIELD_DATA_PP_RX_IND(0xF026, NoParameter),                              //RtxEai
	RTX_EAP_AFIELD_DATA_PP_TX_IND(0xF027, NoParameter),                              //RtxEai
	RTX_EAP_A_B_FULL_SLOT_DATA_FP_RX_IND(0xF028, NoParameter),                       //RtxEai
	RTX_EAP_A_B_FULL_SLOT_DATA_FP_TX_IND(0xF029, NoParameter),                       //RtxEai
	RTX_EAP_A_B_FULL_SLOT_DATA_PP_RX_IND(0xF02A, NoParameter),                       //RtxEai
	RTX_EAP_A_B_FULL_SLOT_DATA_PP_TX_IND(0xF02B, NoParameter),                       //RtxEai
	RTX_EAP_A_B_DOUBLE_SLOT_DATA_FP_RX_IND(0xF02C, NoParameter),                     //RtxEai
	RTX_EAP_A_B_DOUBLE_SLOT_DATA_FP_TX_IND(0xF02D, NoParameter),                     //RtxEai
	RTX_EAP_A_B_DOUBLE_SLOT_DATA_PP_RX_IND(0xF02E, NoParameter),                     //RtxEai
	RTX_EAP_A_B_DOUBLE_SLOT_DATA_PP_TX_IND(0xF02F, NoParameter),                     //RtxEai
	RTX_EAP_RAM_WRITE_REQ(0xF030, NoParameter),                                      //RtxEai
	RTX_EAP_RAM_READ_REQ(0xF031, NoParameter),                                       //RtxEai
	RTX_EAP_RAM_READ_CFM(0xF032, NoParameter),                                       //RtxEai
	RTX_EAP_RAM_WRITE_CFM(0xF033, NoParameter),                                      //RtxEai
	RTX_EAP_RAM_MEMSET_REQ(0xF034, NoParameter),                                     //RtxEai
	RTX_EAP_RAM_MEMSET_CFM(0xF035, NoParameter),                                     //RtxEai
	RTX_EAP_MEMORY_WRITE_WORD_REQ(0xF036, NoParameter),                              //RtxEai
	RTX_EAP_MEMORY_WRITE_WORD_CFM(0xF037, NoParameter),                              //RtxEai
	RTX_EAP_TRACE_INPUT_MAIL2_IND(0xF03C, NoParameter),                              //RtxEai
	RTX_EAP_TRACE_TIME_INPUT_MAIL2_IND(0xF03D, NoParameter),                         //RtxEai
	RTX_EAP_BMC_TRACE_START_REQ(0xF040, NoParameter),                                //RtxEai
	RTX_EAP_BMC_TRACE_START_CFM(0xF041, NoParameter),                                //RtxEai
	RTX_EAP_BMC_TRACE_STOP_REQ(0xF042, NoParameter),                                 //RtxEai
	RTX_EAP_BMC_TRACE_STOP_CFM(0xF043, NoParameter),                                 //RtxEai
	RTX_EAP_BMC_TRACE_MSG_IND(0xF044, NoParameter),                                  //RtxEai
	RTX_EAP_BMC_TRACE_START_REQ_MULTIBEARER(0xF045, NoParameter),                    //RtxEai
	RTX_EAP_BMC_TRACE_MSG_IND_MULTIBEARER(0xF046, NoParameter),                      //RtxEai
	RTX_EAP_BMC_TIME_TRACE_MSG_IND(0xF047, NoParameter),                             //RtxEai
	RTX_EAP_BMC_TIME_TRACE_MSG_IND_MULTIBEARER(0xF048, NoParameter),                 //RtxEai
	TEST_RSSIVUM_TRACE_START_REQ(0xF049, NoParameter),                               //RtxEai
	TEST_RSSIVUM_TRACE_START_CFM(0xF04A, NoParameter),                               //RtxEai
	TEST_RSSIVUM_TRACE_STOP_REQ(0xF04B, NoParameter),                                //RtxEai
	TEST_RSSIVUM_TRACE_STOP_CFM(0xF04C, NoParameter),                                //RtxEai
	TEST_RSSIVUM_TRACE_MSG_IND(0xF04D, NoParameter),                                 //RtxEai
	TEST_RSSIVUM_TRACE_ERRS_MSG_IND(0xF04E, NoParameter),                            //RtxEai
	RTX_EAP_BMC_TRACE_EVENT_IND(0xF04F, NoParameter),                                //RtxEai
	RTX_EAP_BMC_TRACE_TIME_EVENT_IND(0xF050, NoParameter),                           //RtxEai
	RTX_EAP_BMC_TRACE_START_REQ_FP_SYNC(0xF051, NoParameter),                        //RtxEai
	RTX_EAP_TRACE_TIME_START_EXTIMER_IND(0xF054, NoParameter),                       //RtxEai
	RTX_EAP_TRACE_START_EXTIMER_IND(0xF055, NoParameter),                            //RtxEai
	RTX_EAP_TRACE_TIME_STOP_EXTIMER_IND(0xF056, NoParameter),                        //RtxEai
	RTX_EAP_TRACE_STOP_EXTIMER_IND(0xF057, NoParameter),                             //RtxEai
	RTX_EAP_MODULE_TEST_START_REQ(0xF060, NoParameter),                              //RtxEai
	RTX_EAP_MODULE_TEST_STOP_REQ(0xF061, NoParameter),                               //RtxEai
	RTX_EAP_PROJECT_TEST_REQ(0xF062, NoParameter),                                   //RtxEai
	RTX_EAP_PROJECT_TEST_CFM(0xF063, NoParameter),                                   //RtxEai
	RTX_EAP_RF_TEST_REQ(0xF064, NoParameter),                                        //RtxEai
	RTX_EAP_RF_TEST_CFM(0xF065, NoParameter),                                        //RtxEai
	RTX_EAP_HW_TEST_REQ(0xF066, NoParameter),                                        //RtxEai
	RTX_EAP_HW_TEST_CFM(0xF067, NoParameter),                                        //RtxEai
	RTX_EAP_SW_TEST_REQ(0xF068, NoParameter),                                        //RtxEai
	RTX_EAP_SW_TEST_CFM(0xF069, NoParameter),                                        //RtxEai
	RTX_EAP_AUDIO_IND(0xF06A, NoParameter),                                          //RtxEai
	RTX_EAP_PRINTF_START_REQ(0xF070, NoParameter),                                   //RtxEai
	RTX_EAP_PRINTF_START_REQ_NO_CFM(0xF071, NoParameter),                            //RtxEai
	RTX_EAP_PRINTF_START_CFM(0xF072, NoParameter),                                   //RtxEai
	RTX_EAP_PRINTF_STOP_REQ(0xF073, NoParameter),                                    //RtxEai
	RTX_EAP_PRINTF_STOP_CFM(0xF074, NoParameter),                                    //RtxEai
	RTX_EAP_PRINTF_IND(0xF075, NoParameter),                                         //RtxEai
	RTX_EAP_CLEAR_SCREEN_IND(0xF076, NoParameter),                                   //RtxEai
	RTX_EAP_SCREEN_GOTOXY_IND(0xF077, NoParameter),                                  //RtxEai
	RTX_EAP_TRACE_TIME_AFIELD_DATA_FP_RX_IND(0xF0B4, NoParameter),                   //RtxEai
	RTX_EAP_TRACE_TIME_AFIELD_DATA_FP_TX_IND(0xF0B5, NoParameter),                   //RtxEai
	RTX_EAP_TRACE_TIME_AFIELD_DATA_PP_RX_IND(0xF0B6, NoParameter),                   //RtxEai
	RTX_EAP_TRACE_TIME_AFIELD_DATA_PP_TX_IND(0xF0B7, NoParameter),                   //RtxEai
	RTX_EAP_TRACE_TIME_A_B_FULL_SLOT_DATA_FP_RX_IND(0xF0B8, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_FULL_SLOT_DATA_FP_TX_IND(0xF0B9, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_FULL_SLOT_DATA_PP_RX_IND(0xF0BA, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_FULL_SLOT_DATA_PP_TX_IND(0xF0BB, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_DOUBLE_SLOT_DATA_FP_RX_IND(0xF0BC, NoParameter),          //RtxEai
	RTX_EAP_TRACE_TIME_A_B_DOUBLE_SLOT_DATA_FP_TX_IND(0xF0BD, NoParameter),          //RtxEai
	RTX_EAP_TRACE_TIME_A_B_DOUBLE_SLOT_DATA_PP_RX_IND(0xF0BE, NoParameter),          //RtxEai
	RTX_EAP_TRACE_TIME_A_B_DOUBLE_SLOT_DATA_PP_TX_IND(0xF0BF, NoParameter),          //RtxEai
	RTX_EAP_TRACE_TIME_PDU_IND(0xF0C0, NoParameter),                                 //RtxEai
	RTX_EAP_TRACE_TIME_FUNCTION_IND(0xF0C1, NoParameter),                            //RtxEai
	RTX_EAP_TRACE_TIME_DATA_IND(0xF0C2, NoParameter),                                //RtxEai
	RTX_EAP_TRACE_TIME_IND(0xF0C3, NoParameter),                                     //RtxEai
	RTX_EAP_TRACE_TIME_A_B_LONG_SLOT_DATA_FP_RX_IND(0xF0C4, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_LONG_SLOT_DATA_FP_TX_IND(0xF0C5, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_LONG_SLOT_DATA_PP_RX_IND(0xF0C6, NoParameter),            //RtxEai
	RTX_EAP_TRACE_TIME_A_B_LONG_SLOT_DATA_PP_TX_IND(0xF0C7, NoParameter),            //RtxEai
	RTX_EAP_A_B_LONG_SLOT_DATA_FP_RX_IND(0xF0C8, NoParameter),                       //RtxEai
	RTX_EAP_A_B_LONG_SLOT_DATA_FP_TX_IND(0xF0C9, NoParameter),                       //RtxEai
	RTX_EAP_A_B_LONG_SLOT_DATA_PP_RX_IND(0xF0CA, NoParameter),                       //RtxEai
	RTX_EAP_A_B_LONG_SLOT_DATA_PP_TX_IND(0xF0CB, NoParameter),                       //RtxEai
	RTX_EAP_MEMORY_READ_REQ(0xF0E0, NoParameter),                                    //RtxEai
	RTX_EAP_MEMORY_READ_CFM(0xF0E1, NoParameter),                                    //RtxEai
	RTX_EAP_MEMORY_WRITE_REQ(0xF0E2, NoParameter),                                   //RtxEai
	RTX_EAP_MEMORY_WRITE_CFM(0xF0E3, NoParameter),                                   //RtxEai
	RTX_EAP_MEMORY_MEMSET_REQ(0xF0E4, NoParameter),                                  //RtxEai
	RTX_EAP_MEMORY_MEMSET_CFM(0xF0E5, NoParameter),                                  //RtxEai
	RTX_EAP_MEMORY_SET_DEFAULT_REQ(0xF0E6, NoParameter),                             //RtxEai
	RTX_EAP_MEMORY_SET_DEFAULT_CFM(0xF0E7, NoParameter),                             //RtxEai
	RTX_EAP_VERSION_REQ(0xF0F0, NoParameter),                                        //RtxEai
	RTX_EAP_VERSION_CFM(0xF0F1, NoParameter),                                        //RtxEai
	RTX_EAP_SW_BUILD_INFO_REQ(0xF0F2, NoParameter),                                  //RtxEai
	RTX_EAP_SW_BUILD_INFO_CFM(0xF0F3, NoParameter),                                  //RtxEai
	RTX_EAP_TARGET_RESET_IND(0xF0F4, NoParameter),                                   //RtxEai
	RTX_EAP_PACKET_CFM(0xF0F5, NoParameter),                                         //RtxEai
	RTX_EAP_PACKET_ERROR_IND(0xF0F6, NoParameter),                                   //RtxEai
	RTX_EAP_NVS_CACHE_EMPTY_REQ(0xF117, NoParameter),                                //RtxEai
	RTX_EAP_NVS_CACHE_EMPTY_CFM(0xF118, NoParameter),                                //RtxEai
	RTX_EAP_NVS_SET_DEFAULT_REQ(0xF119, NoParameter),                                //RtxEai
	RTX_EAP_NVS_SET_DEFAULT_CFM(0xF11A, NoParameter),                                //RtxEai
	RTX_EAP_NVS_READ_REQ(0xF11B, NoParameter),                                       //RtxEai
	RTX_EAP_NVS_READ_CFM(0xF11C, NoParameter),                                       //RtxEai
	RTX_EAP_NVS_WRITE_REQ(0xF11D, NoParameter),                                      //RtxEai
	RTX_EAP_NVS_WRITE_CFM(0xF11E, NoParameter),                                      //RtxEai
	RTX_EAP_NVS_SET_DOMAIN_REQ(0xF11F, NoParameter),                                 //RtxEai
	RTX_EAP_NVS_SET_DOMAIN_CFM(0xF120, NoParameter),                                 //RtxEai
	RTX_EAP_NVS_GET_DOMAIN_SIZE_REQ(0xF121, NoParameter),                            //RtxEai
	RTX_EAP_NVS_GET_DOMAIN_SIZE_CFM(0xF122, NoParameter),                            //RtxEai
	RTX_EAP_TRACESERVER_GET_LIST_REQ(0xF180, NoParameter),                           //RtxEai
	RTX_EAP_TRACESERVER_GET_LIST_CFM(0xF181, NoParameter),                           //RtxEai
	RTX_EAP_TRACESERVER_SELECT_TARGET_REQ(0xF182, NoParameter),                      //RtxEai
	RTX_EAP_TRACESERVER_SELECT_TARGET_CFM(0xF183, NoParameter),                      //RtxEai
	RTX_EAP_TRACESERVER_SELECT_TARGET_REJ(0xF184, NoParameter),                      //RtxEai
	RTX_EAP_TRACESERVER_TARGET_DESC_IND(0xF185, NoParameter),                        //RtxEai
	RTX_EAP_TRACESERVER_CONNECT_TARGET_REQ(0xF186, NoParameter),                     //RtxEai
	RTX_EAP_TRACESERVER_CONNECT_TARGET_CFM(0xF187, NoParameter),                     //RtxEai
	RTX_EAP_TRACESERVER_CONNECT_TARGET_REJ(0xF188, NoParameter),                     //RtxEai
	RTX_EAP_TRACE_CREATE_RT_TIMER_IND(0xF1A0, NoParameter),                          //RtxEai
	RTX_EAP_TRACE_DESTROY_RT_TIMER_IND(0xF1A1, NoParameter),                         //RtxEai
	RTX_EAP_TRACE_START_RT_TIMER_IND(0xF1A2, NoParameter),                           //RtxEai
	RTX_EAP_TRACE_STOP_RT_TIMER_IND(0xF1A3, NoParameter),                            //RtxEai
	RTX_EAP_TRACE_MEM_ALLOC_IND(0xF1B0, NoParameter),                                //RtxEai
	RTX_EAP_TRACE_MEM_FREE_IND(0xF1B1, NoParameter),                                 //RtxEai
	RTX_EAP_TRACE_ADD_WATCH_IND(0xF1B2, NoParameter),                                //RtxEai
	RTX_EAP_GRAPH_DATA_IND(0xF1B3, NoParameter),                                     //RtxEai
	RTX_EAP_GRAPH_UPDATE_IND(0xF1B4, NoParameter),                                   //RtxEai
	RTX_EAP_KEY_PRESS_REQ(0xF1B5, NoParameter),                                      //RtxEai
	RTX_EAP_KEY_PRESS_CFM(0xF1B6, NoParameter),                                      //RtxEai
	RTX_EAP_ADD_GRAPH_IND(0xF1B7, NoParameter),                                      //RtxEai
	RTX_EAP_ADD_LINE_TO_GRAPH_IND(0xF1B8, NoParameter),                              //RtxEai
	RTX_EAP_REMOVE_GRAPH_IND(0xF1B9, NoParameter),                                   //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_START_CFM(0xF1BA, NoParameter),                         //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_START_REJ(0xF1BB, NoParameter),                         //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_INFO_REQ(0xF1BC, NoParameter),                          //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_INFO_CFM(0xF1BD, NoParameter),                          //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_UPDATE_IND(0xF1BE, NoParameter),                        //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_START_REQ(0xF1BF, NoParameter),                         //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_STOP_REQ(0xF1C0, NoParameter),                          //RtxEai
	RTX_EAP_FPSYNC_SLAVE_DBG_STOP_CFM(0xF1C1, NoParameter),                          //RtxEai
	MOD_TEST_INIT_REQ(0xFA00, NoParameter),                                          //ModTest
	MOD_TEST_CLOSE_REQ(0xFA01, NoParameter),                                         //ModTest
	MOD_TEST_DEBUG_REQ(0xFA02, NoParameter),                                         //ModTest
	MOD_TEST_INIT_CFM(0xFA03, NoParameter),                                          //ModTest
	MOD_TEST_CLOSE_CFM(0xFA04, NoParameter),                                         //ModTest
	MOD_TEST_SYSCFG_REQ(0xFA05, NoParameter),                                        //ModTest
	MOD_TEST_SYSCFG_CFM(0xFA06, NoParameter),                                        //ModTest
	MOD_TEST_START_TIMER_IND(0xFA07, NoParameter),                                   //ModTest
	MOD_TEST_STOP_TIMER_IND(0xFA08, NoParameter),                                    //ModTest
	MOD_TEST_READ_EEPROM_IND(0xFA09, NoParameter),                                   //ModTest
	MOD_TEST_WRITE_EEPROM_IND(0xFA0A, NoParameter),                                  //ModTest
	MOD_TEST_COMMENT_REQ(0xFA0B, NoParameter),                                       //ModTest
	MOD_TEST_GET_DATA_REQ(0xFA0C, NoParameter),                                      //ModTest
	MOD_TEST_GET_DATA_CFM(0xFA0D, NoParameter),                                      //ModTest
	MOD_TEST_PUT_DATA_REQ(0xFA0E, NoParameter),                                      //ModTest
	MOD_TEST_PUT_DATA_CFM(0xFA0F, NoParameter),                                      //ModTest
	MOD_TEST_CLEAN_DATA_LIST_REQ(0xFA10, NoParameter),                               //ModTest
	MOD_TEST_CLEAN_DATA_LIST_CFM(0xFA11, NoParameter),                               //ModTest
	MOD_TEST_EXPIRE_DYNAMIC_TIMER_REQ(0xFA12, NoParameter),                          //ModTest
	MOD_TEST_START_DYNAMIC_TIMER_IND(0xFA13, NoParameter),                           //ModTest
	MOD_TEST_STOP_DYNAMIC_TIMER_IND(0xFA14, NoParameter),                            //ModTest
	MOD_TEST_RESET_REQ(0xFA15, NoParameter),                                         //ModTest
	MOD_TEST_RESET_CFM(0xFA16, NoParameter),                                         //ModTest
	RTX_EAP_TEST_CONNECTION_PING_PONG_IND(0xFF00, NoParameter),                      //RtxEai
}
