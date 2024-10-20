#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use dlopen2::wrapper::WrapperApi;

#[derive(WrapperApi)]
pub struct VisaFFI {
    viOpenDefaultRM: fn(vi: ViPSession) -> ViStatus,
    viFindRsrc: fn(
        sesn: ViSession,
        expr: ViConstString,
        vi: ViPFindList,
        retCnt: ViPUInt32,
        desc: *mut ViChar,
    ) -> ViStatus,
    viFindNext: fn(vi: ViFindList, desc: *mut ViChar) -> ViStatus,
    viParseRsrc: fn(
        rmSesn: ViSession,
        rsrcName: ViConstRsrc,
        intfType: ViPUInt16,
        intfNum: ViPUInt16,
    ) -> ViStatus,
    viParseRsrcEx: fn(
        rmSesn: ViSession,
        rsrcName: ViConstRsrc,
        intfType: ViPUInt16,
        intfNum: ViPUInt16,
        rsrcClass: *mut ViChar,
        expandedUnaliasedName: *mut ViChar,
        aliasIfExists: *mut ViChar,
    ) -> ViStatus,
    viOpen: fn(
        sesn: ViSession,
        name: ViConstRsrc,
        mode: ViAccessMode,
        timeout: ViUInt32,
        vi: ViPSession,
    ) -> ViStatus,
    viClose: fn(vi: ViObject) -> ViStatus,
    viSetAttribute: fn(vi: ViObject, attrName: ViAttr, attrValue: ViAttrState) -> ViStatus,
    viGetAttribute:
        fn(vi: ViObject, attrName: ViAttr, attrValue: *mut ::std::os::raw::c_void) -> ViStatus,
    viStatusDesc: fn(vi: ViObject, status: ViStatus, desc: *mut ViByte) -> ViStatus,
    viTerminate: fn(vi: ViObject, degree: ViUInt16, jobId: ViJobId) -> ViStatus,
    viLock: fn(
        vi: ViSession,
        lockType: ViAccessMode,
        timeout: ViUInt32,
        requestedKey: ViConstKeyId,
        accessKey: *mut ViChar,
    ) -> ViStatus,
    viUnlock: fn(vi: ViSession) -> ViStatus,
    viEnableEvent: fn(
        vi: ViSession,
        eventType: ViEventType,
        mechanism: ViUInt16,
        context: ViEventFilter,
    ) -> ViStatus,
    viDisableEvent: fn(vi: ViSession, eventType: ViEventType, mechanism: ViUInt16) -> ViStatus,
    viDiscardEvents: fn(vi: ViSession, eventType: ViEventType, mechanism: ViUInt16) -> ViStatus,
    viWaitOnEvent: fn(
        vi: ViSession,
        inEventType: ViEventType,
        timeout: ViUInt32,
        outEventType: ViPEventType,
        outContext: ViPEvent,
    ) -> ViStatus,
    viInstallHandler:
        fn(vi: ViSession, eventType: ViEventType, handler: ViHndlr, userHandle: ViAddr) -> ViStatus,
    viUninstallHandler:
        fn(vi: ViSession, eventType: ViEventType, handler: ViHndlr, userHandle: ViAddr) -> ViStatus,
    viRead: fn(vi: ViSession, buf: ViPBuf, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viReadAsync: fn(vi: ViSession, buf: ViPBuf, cnt: ViUInt32, jobId: ViPJobId) -> ViStatus,
    viReadToFile:
        fn(vi: ViSession, filename: ViConstString, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viWrite: fn(vi: ViSession, buf: ViConstBuf, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viWriteAsync: fn(vi: ViSession, buf: ViConstBuf, cnt: ViUInt32, jobId: ViPJobId) -> ViStatus,
    viWriteFromFile:
        fn(vi: ViSession, filename: ViConstString, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viAssertTrigger: fn(vi: ViSession, protocol: ViUInt16) -> ViStatus,
    viReadSTB: fn(vi: ViSession, status: ViPUInt16) -> ViStatus,
    viClear: fn(vi: ViSession) -> ViStatus,
    viSetBuf: fn(vi: ViSession, mask: ViUInt16, size: ViUInt32) -> ViStatus,
    viFlush: fn(vi: ViSession, mask: ViUInt16) -> ViStatus,
    viBufWrite: fn(vi: ViSession, buf: ViConstBuf, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viBufRead: fn(vi: ViSession, buf: ViPBuf, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viVPrintf: fn(vi: ViSession, writeFmt: ViConstString, params: ViVAList) -> ViStatus,
    viVSPrintf:
        fn(vi: ViSession, buf: ViPBuf, writeFmt: ViConstString, parms: ViVAList) -> ViStatus,
    viVScanf: fn(vi: ViSession, readFmt: ViConstString, params: ViVAList) -> ViStatus,
    viVSScanf:
        fn(vi: ViSession, buf: ViConstBuf, readFmt: ViConstString, parms: ViVAList) -> ViStatus,
    viVQueryf: fn(
        vi: ViSession,
        writeFmt: ViConstString,
        readFmt: ViConstString,
        params: ViVAList,
    ) -> ViStatus,
    viIn8: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val8: ViPUInt8) -> ViStatus,
    viOut8: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val8: ViUInt8) -> ViStatus,
    viIn16: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val16: ViPUInt16) -> ViStatus,
    viOut16: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val16: ViUInt16) -> ViStatus,
    viIn32: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val32: ViPUInt32) -> ViStatus,
    viOut32: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val32: ViUInt32) -> ViStatus,
    viIn64: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val64: ViPUInt64) -> ViStatus,
    viOut64: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress, val64: ViUInt64) -> ViStatus,
    viIn8Ex: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val8: ViPUInt8) -> ViStatus,
    viOut8Ex: fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val8: ViUInt8) -> ViStatus,
    viIn16Ex:
        fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val16: ViPUInt16) -> ViStatus,
    viOut16Ex:
        fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val16: ViUInt16) -> ViStatus,
    viIn32Ex:
        fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val32: ViPUInt32) -> ViStatus,
    viOut32Ex:
        fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val32: ViUInt32) -> ViStatus,
    viIn64Ex:
        fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val64: ViPUInt64) -> ViStatus,
    viOut64Ex:
        fn(vi: ViSession, space: ViUInt16, offset: ViBusAddress64, val64: ViUInt64) -> ViStatus,
    viMoveIn8: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf8: ViAUInt8,
    ) -> ViStatus,
    viMoveOut8: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf8: ViAUInt8,
    ) -> ViStatus,
    viMoveIn16: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf16: ViAUInt16,
    ) -> ViStatus,
    viMoveOut16: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf16: ViAUInt16,
    ) -> ViStatus,
    viMoveIn32: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf32: ViAUInt32,
    ) -> ViStatus,
    viMoveOut32: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf32: ViAUInt32,
    ) -> ViStatus,
    viMoveIn64: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf64: ViAUInt64,
    ) -> ViStatus,
    viMoveOut64: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress,
        length: ViBusSize,
        buf64: ViAUInt64,
    ) -> ViStatus,
    viMoveIn8Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf8: ViAUInt8,
    ) -> ViStatus,
    viMoveOut8Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf8: ViAUInt8,
    ) -> ViStatus,
    viMoveIn16Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf16: ViAUInt16,
    ) -> ViStatus,
    viMoveOut16Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf16: ViAUInt16,
    ) -> ViStatus,
    viMoveIn32Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf32: ViAUInt32,
    ) -> ViStatus,
    viMoveOut32Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf32: ViAUInt32,
    ) -> ViStatus,
    viMoveIn64Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf64: ViAUInt64,
    ) -> ViStatus,
    viMoveOut64Ex: fn(
        vi: ViSession,
        space: ViUInt16,
        offset: ViBusAddress64,
        length: ViBusSize,
        buf64: ViAUInt64,
    ) -> ViStatus,
    viMove: fn(
        vi: ViSession,
        srcSpace: ViUInt16,
        srcOffset: ViBusAddress,
        srcWidth: ViUInt16,
        destSpace: ViUInt16,
        destOffset: ViBusAddress,
        destWidth: ViUInt16,
        srcLength: ViBusSize,
    ) -> ViStatus,
    viMoveAsync: fn(
        vi: ViSession,
        srcSpace: ViUInt16,
        srcOffset: ViBusAddress,
        srcWidth: ViUInt16,
        destSpace: ViUInt16,
        destOffset: ViBusAddress,
        destWidth: ViUInt16,
        srcLength: ViBusSize,
        jobId: ViPJobId,
    ) -> ViStatus,
    viMoveEx: fn(
        vi: ViSession,
        srcSpace: ViUInt16,
        srcOffset: ViBusAddress64,
        srcWidth: ViUInt16,
        destSpace: ViUInt16,
        destOffset: ViBusAddress64,
        destWidth: ViUInt16,
        srcLength: ViBusSize,
    ) -> ViStatus,
    viMoveAsyncEx: fn(
        vi: ViSession,
        srcSpace: ViUInt16,
        srcOffset: ViBusAddress64,
        srcWidth: ViUInt16,
        destSpace: ViUInt16,
        destOffset: ViBusAddress64,
        destWidth: ViUInt16,
        srcLength: ViBusSize,
        jobId: ViPJobId,
    ) -> ViStatus,
    viMapAddress: fn(
        vi: ViSession,
        mapSpace: ViUInt16,
        mapOffset: ViBusAddress,
        mapSize: ViBusSize,
        access: ViBoolean,
        suggested: ViAddr,
        address: ViPAddr,
    ) -> ViStatus,
    viUnmapAddress: fn(vi: ViSession) -> ViStatus,
    viMapAddressEx: fn(
        vi: ViSession,
        mapSpace: ViUInt16,
        mapOffset: ViBusAddress64,
        mapSize: ViBusSize,
        access: ViBoolean,
        suggested: ViAddr,
        address: ViPAddr,
    ) -> ViStatus,
    viPeek8: fn(vi: ViSession, address: ViAddr, val8: ViPUInt8),
    viPoke8: fn(vi: ViSession, address: ViAddr, val8: ViUInt8),
    viPeek16: fn(vi: ViSession, address: ViAddr, val16: ViPUInt16),
    viPoke16: fn(vi: ViSession, address: ViAddr, val16: ViUInt16),
    viPeek32: fn(vi: ViSession, address: ViAddr, val32: ViPUInt32),
    viPoke32: fn(vi: ViSession, address: ViAddr, val32: ViUInt32),
    viPeek64: fn(vi: ViSession, address: ViAddr, val64: ViPUInt64),
    viPoke64: fn(vi: ViSession, address: ViAddr, val64: ViUInt64),
    viMemAlloc: fn(vi: ViSession, size: ViBusSize, offset: ViPBusAddress) -> ViStatus,
    viMemFree: fn(vi: ViSession, offset: ViBusAddress) -> ViStatus,
    viMemAllocEx: fn(vi: ViSession, size: ViBusSize, offset: ViPBusAddress64) -> ViStatus,
    viMemFreeEx: fn(vi: ViSession, offset: ViBusAddress64) -> ViStatus,
    viGpibControlREN: fn(vi: ViSession, mode: ViUInt16) -> ViStatus,
    viGpibControlATN: fn(vi: ViSession, mode: ViUInt16) -> ViStatus,
    viGpibSendIFC: fn(vi: ViSession) -> ViStatus,
    viGpibCommand: fn(vi: ViSession, cmd: ViConstBuf, cnt: ViUInt32, retCnt: ViPUInt32) -> ViStatus,
    viGpibPassControl: fn(vi: ViSession, primAddr: ViUInt16, secAddr: ViUInt16) -> ViStatus,
    viVxiCommandQuery:
        fn(vi: ViSession, mode: ViUInt16, cmd: ViUInt32, response: ViPUInt32) -> ViStatus,
    viAssertUtilSignal: fn(vi: ViSession, line: ViUInt16) -> ViStatus,
    viAssertIntrSignal: fn(vi: ViSession, mode: ViInt16, statusID: ViUInt32) -> ViStatus,
    viMapTrigger:
        fn(vi: ViSession, trigSrc: ViInt16, trigDest: ViInt16, mode: ViUInt16) -> ViStatus,
    viUnmapTrigger: fn(vi: ViSession, trigSrc: ViInt16, trigDest: ViInt16) -> ViStatus,
    viUsbControlOut: fn(
        vi: ViSession,
        bmRequestType: ViInt16,
        bRequest: ViInt16,
        wValue: ViUInt16,
        wIndex: ViUInt16,
        wLength: ViUInt16,
        buf: ViConstBuf,
    ) -> ViStatus,
    viUsbControlIn: fn(
        vi: ViSession,
        bmRequestType: ViInt16,
        bRequest: ViInt16,
        wValue: ViUInt16,
        wIndex: ViUInt16,
        wLength: ViUInt16,
        buf: ViPBuf,
        retCnt: ViPUInt16,
    ) -> ViStatus,
    viPxiReserveTriggers: fn(
        vi: ViSession,
        cnt: ViInt16,
        trigBuses: ViAInt16,
        trigLines: ViAInt16,
        failureIndex: ViPInt16,
    ) -> ViStatus,
}

/* All of the declarations are from visa-sys (thanks TsuITOAR) They saved me a lot of work.
I took them to get started quickly and will be optimized in the future to be cleaner.*/

/* some platform specified defines adjusted by hand */
pub const __GNUC_VA_LIST: u32 = 1;
pub const _VI_ERROR: i32 = -2147483648;
pub const VI_SUCCESS: u32 = 0;
pub const VI_NULL: u32 = 0;
pub const VI_TRUE: u32 = 1;
pub const VI_FALSE: u32 = 0;
pub const VI_SPEC_VERSION: u32 = 7340288;
pub const VI_ATTR_RSRC_CLASS: u32 = 3221159937;
pub const VI_ATTR_RSRC_NAME: u32 = 3221159938;
pub const VI_ATTR_RSRC_IMPL_VERSION: u32 = 1073676291;
pub const VI_ATTR_RSRC_LOCK_STATE: u32 = 1073676292;
pub const VI_ATTR_MAX_QUEUE_LENGTH: u32 = 1073676293;
pub const VI_ATTR_USER_DATA_32: u32 = 1073676295;
pub const VI_ATTR_FDC_CHNL: u32 = 1073676301;
pub const VI_ATTR_FDC_MODE: u32 = 1073676303;
pub const VI_ATTR_FDC_GEN_SIGNAL_EN: u32 = 1073676305;
pub const VI_ATTR_FDC_USE_PAIR: u32 = 1073676307;
pub const VI_ATTR_SEND_END_EN: u32 = 1073676310;
pub const VI_ATTR_TERMCHAR: u32 = 1073676312;
pub const VI_ATTR_TMO_VALUE: u32 = 1073676314;
pub const VI_ATTR_GPIB_READDR_EN: u32 = 1073676315;
pub const VI_ATTR_IO_PROT: u32 = 1073676316;
pub const VI_ATTR_DMA_ALLOW_EN: u32 = 1073676318;
pub const VI_ATTR_ASRL_BAUD: u32 = 1073676321;
pub const VI_ATTR_ASRL_DATA_BITS: u32 = 1073676322;
pub const VI_ATTR_ASRL_PARITY: u32 = 1073676323;
pub const VI_ATTR_ASRL_STOP_BITS: u32 = 1073676324;
pub const VI_ATTR_ASRL_FLOW_CNTRL: u32 = 1073676325;
pub const VI_ATTR_RD_BUF_OPER_MODE: u32 = 1073676330;
pub const VI_ATTR_RD_BUF_SIZE: u32 = 1073676331;
pub const VI_ATTR_WR_BUF_OPER_MODE: u32 = 1073676333;
pub const VI_ATTR_WR_BUF_SIZE: u32 = 1073676334;
pub const VI_ATTR_SUPPRESS_END_EN: u32 = 1073676342;
pub const VI_ATTR_TERMCHAR_EN: u32 = 1073676344;
pub const VI_ATTR_DEST_ACCESS_PRIV: u32 = 1073676345;
pub const VI_ATTR_DEST_BYTE_ORDER: u32 = 1073676346;
pub const VI_ATTR_SRC_ACCESS_PRIV: u32 = 1073676348;
pub const VI_ATTR_SRC_BYTE_ORDER: u32 = 1073676349;
pub const VI_ATTR_SRC_INCREMENT: u32 = 1073676352;
pub const VI_ATTR_DEST_INCREMENT: u32 = 1073676353;
pub const VI_ATTR_WIN_ACCESS_PRIV: u32 = 1073676357;
pub const VI_ATTR_WIN_BYTE_ORDER: u32 = 1073676359;
pub const VI_ATTR_GPIB_ATN_STATE: u32 = 1073676375;
pub const VI_ATTR_GPIB_ADDR_STATE: u32 = 1073676380;
pub const VI_ATTR_GPIB_CIC_STATE: u32 = 1073676382;
pub const VI_ATTR_GPIB_NDAC_STATE: u32 = 1073676386;
pub const VI_ATTR_GPIB_SRQ_STATE: u32 = 1073676391;
pub const VI_ATTR_GPIB_SYS_CNTRL_STATE: u32 = 1073676392;
pub const VI_ATTR_GPIB_HS488_CBL_LEN: u32 = 1073676393;
pub const VI_ATTR_CMDR_LA: u32 = 1073676395;
pub const VI_ATTR_VXI_DEV_CLASS: u32 = 1073676396;
pub const VI_ATTR_MAINFRAME_LA: u32 = 1073676400;
pub const VI_ATTR_MANF_NAME: u32 = 3221160050;
pub const VI_ATTR_MODEL_NAME: u32 = 3221160055;
pub const VI_ATTR_VXI_VME_INTR_STATUS: u32 = 1073676427;
pub const VI_ATTR_VXI_TRIG_STATUS: u32 = 1073676429;
pub const VI_ATTR_VXI_VME_SYSFAIL_STATE: u32 = 1073676436;
pub const VI_ATTR_WIN_BASE_ADDR_32: u32 = 1073676440;
pub const VI_ATTR_WIN_SIZE_32: u32 = 1073676442;
pub const VI_ATTR_ASRL_AVAIL_NUM: u32 = 1073676460;
pub const VI_ATTR_MEM_BASE_32: u32 = 1073676461;
pub const VI_ATTR_ASRL_CTS_STATE: u32 = 1073676462;
pub const VI_ATTR_ASRL_DCD_STATE: u32 = 1073676463;
pub const VI_ATTR_ASRL_DSR_STATE: u32 = 1073676465;
pub const VI_ATTR_ASRL_DTR_STATE: u32 = 1073676466;
pub const VI_ATTR_ASRL_END_IN: u32 = 1073676467;
pub const VI_ATTR_ASRL_END_OUT: u32 = 1073676468;
pub const VI_ATTR_ASRL_REPLACE_CHAR: u32 = 1073676478;
pub const VI_ATTR_ASRL_RI_STATE: u32 = 1073676479;
pub const VI_ATTR_ASRL_RTS_STATE: u32 = 1073676480;
pub const VI_ATTR_ASRL_XON_CHAR: u32 = 1073676481;
pub const VI_ATTR_ASRL_XOFF_CHAR: u32 = 1073676482;
pub const VI_ATTR_WIN_ACCESS: u32 = 1073676483;
pub const VI_ATTR_RM_SESSION: u32 = 1073676484;
pub const VI_ATTR_VXI_LA: u32 = 1073676501;
pub const VI_ATTR_MANF_ID: u32 = 1073676505;
pub const VI_ATTR_MEM_SIZE_32: u32 = 1073676509;
pub const VI_ATTR_MEM_SPACE: u32 = 1073676510;
pub const VI_ATTR_MODEL_CODE: u32 = 1073676511;
pub const VI_ATTR_SLOT: u32 = 1073676520;
pub const VI_ATTR_INTF_INST_NAME: u32 = 3221160169;
pub const VI_ATTR_IMMEDIATE_SERV: u32 = 1073676544;
pub const VI_ATTR_INTF_PARENT_NUM: u32 = 1073676545;
pub const VI_ATTR_RSRC_SPEC_VERSION: u32 = 1073676656;
pub const VI_ATTR_INTF_TYPE: u32 = 1073676657;
pub const VI_ATTR_GPIB_PRIMARY_ADDR: u32 = 1073676658;
pub const VI_ATTR_GPIB_SECONDARY_ADDR: u32 = 1073676659;
pub const VI_ATTR_RSRC_MANF_NAME: u32 = 3221160308;
pub const VI_ATTR_RSRC_MANF_ID: u32 = 1073676661;
pub const VI_ATTR_INTF_NUM: u32 = 1073676662;
pub const VI_ATTR_TRIG_ID: u32 = 1073676663;
pub const VI_ATTR_GPIB_REN_STATE: u32 = 1073676673;
pub const VI_ATTR_GPIB_UNADDR_EN: u32 = 1073676676;
pub const VI_ATTR_DEV_STATUS_BYTE: u32 = 1073676681;
pub const VI_ATTR_FILE_APPEND_EN: u32 = 1073676690;
pub const VI_ATTR_VXI_TRIG_SUPPORT: u32 = 1073676692;
pub const VI_ATTR_TCPIP_ADDR: u32 = 3221160341;
pub const VI_ATTR_TCPIP_HOSTNAME: u32 = 3221160342;
pub const VI_ATTR_TCPIP_PORT: u32 = 1073676695;
pub const VI_ATTR_TCPIP_DEVICE_NAME: u32 = 3221160345;
pub const VI_ATTR_TCPIP_NODELAY: u32 = 1073676698;
pub const VI_ATTR_TCPIP_KEEPALIVE: u32 = 1073676699;
pub const VI_ATTR_4882_COMPLIANT: u32 = 1073676703;
pub const VI_ATTR_USB_SERIAL_NUM: u32 = 3221160352;
pub const VI_ATTR_USB_INTFC_NUM: u32 = 1073676705;
pub const VI_ATTR_USB_PROTOCOL: u32 = 1073676711;
pub const VI_ATTR_USB_MAX_INTR_SIZE: u32 = 1073676719;
pub const VI_ATTR_PXI_DEV_NUM: u32 = 1073676801;
pub const VI_ATTR_PXI_FUNC_NUM: u32 = 1073676802;
pub const VI_ATTR_PXI_BUS_NUM: u32 = 1073676805;
pub const VI_ATTR_PXI_CHASSIS: u32 = 1073676806;
pub const VI_ATTR_PXI_SLOTPATH: u32 = 3221160455;
pub const VI_ATTR_PXI_SLOT_LBUS_LEFT: u32 = 1073676808;
pub const VI_ATTR_PXI_SLOT_LBUS_RIGHT: u32 = 1073676809;
pub const VI_ATTR_PXI_TRIG_BUS: u32 = 1073676810;
pub const VI_ATTR_PXI_STAR_TRIG_BUS: u32 = 1073676811;
pub const VI_ATTR_PXI_STAR_TRIG_LINE: u32 = 1073676812;
pub const VI_ATTR_PXI_SRC_TRIG_BUS: u32 = 1073676813;
pub const VI_ATTR_PXI_DEST_TRIG_BUS: u32 = 1073676814;
pub const VI_ATTR_PXI_MEM_TYPE_BAR0: u32 = 1073676817;
pub const VI_ATTR_PXI_MEM_TYPE_BAR1: u32 = 1073676818;
pub const VI_ATTR_PXI_MEM_TYPE_BAR2: u32 = 1073676819;
pub const VI_ATTR_PXI_MEM_TYPE_BAR3: u32 = 1073676820;
pub const VI_ATTR_PXI_MEM_TYPE_BAR4: u32 = 1073676821;
pub const VI_ATTR_PXI_MEM_TYPE_BAR5: u32 = 1073676822;
pub const VI_ATTR_PXI_MEM_BASE_BAR0_32: u32 = 1073676833;
pub const VI_ATTR_PXI_MEM_BASE_BAR1_32: u32 = 1073676834;
pub const VI_ATTR_PXI_MEM_BASE_BAR2_32: u32 = 1073676835;
pub const VI_ATTR_PXI_MEM_BASE_BAR3_32: u32 = 1073676836;
pub const VI_ATTR_PXI_MEM_BASE_BAR4_32: u32 = 1073676837;
pub const VI_ATTR_PXI_MEM_BASE_BAR5_32: u32 = 1073676838;
pub const VI_ATTR_PXI_MEM_BASE_BAR0_64: u32 = 1073676840;
pub const VI_ATTR_PXI_MEM_BASE_BAR1_64: u32 = 1073676841;
pub const VI_ATTR_PXI_MEM_BASE_BAR2_64: u32 = 1073676842;
pub const VI_ATTR_PXI_MEM_BASE_BAR3_64: u32 = 1073676843;
pub const VI_ATTR_PXI_MEM_BASE_BAR4_64: u32 = 1073676844;
pub const VI_ATTR_PXI_MEM_BASE_BAR5_64: u32 = 1073676845;
pub const VI_ATTR_PXI_MEM_SIZE_BAR0_32: u32 = 1073676849;
pub const VI_ATTR_PXI_MEM_SIZE_BAR1_32: u32 = 1073676850;
pub const VI_ATTR_PXI_MEM_SIZE_BAR2_32: u32 = 1073676851;
pub const VI_ATTR_PXI_MEM_SIZE_BAR3_32: u32 = 1073676852;
pub const VI_ATTR_PXI_MEM_SIZE_BAR4_32: u32 = 1073676853;
pub const VI_ATTR_PXI_MEM_SIZE_BAR5_32: u32 = 1073676854;
pub const VI_ATTR_PXI_MEM_SIZE_BAR0_64: u32 = 1073676856;
pub const VI_ATTR_PXI_MEM_SIZE_BAR1_64: u32 = 1073676857;
pub const VI_ATTR_PXI_MEM_SIZE_BAR2_64: u32 = 1073676858;
pub const VI_ATTR_PXI_MEM_SIZE_BAR3_64: u32 = 1073676859;
pub const VI_ATTR_PXI_MEM_SIZE_BAR4_64: u32 = 1073676860;
pub const VI_ATTR_PXI_MEM_SIZE_BAR5_64: u32 = 1073676861;
pub const VI_ATTR_PXI_IS_EXPRESS: u32 = 1073676864;
pub const VI_ATTR_PXI_SLOT_LWIDTH: u32 = 1073676865;
pub const VI_ATTR_PXI_MAX_LWIDTH: u32 = 1073676866;
pub const VI_ATTR_PXI_ACTUAL_LWIDTH: u32 = 1073676867;
pub const VI_ATTR_PXI_DSTAR_BUS: u32 = 1073676868;
pub const VI_ATTR_PXI_DSTAR_SET: u32 = 1073676869;
pub const VI_ATTR_PXI_ALLOW_WRITE_COMBINE: u32 = 1073676870;
pub const VI_ATTR_TCPIP_SERVER_CERT_ISSUER_NAME: u32 = 3221160560;
pub const VI_ATTR_TCPIP_SERVER_CERT_SUBJECT_NAME: u32 = 3221160561;
pub const VI_ATTR_TCPIP_SERVER_CERT_EXPIRATION_DATE: u32 = 3221160562;
pub const VI_ATTR_TCPIP_SERVER_CERT_IS_PERPETUAL: u32 = 1073676915;
pub const VI_ATTR_TCPIP_SASL_MECHANISM: u32 = 3221160564;
pub const VI_ATTR_TCPIP_TLS_CIPHER_SUITE: u32 = 3221160565;
pub const VI_ATTR_TCPIP_HISLIP_OVERLAP_EN: u32 = 1073677056;
pub const VI_ATTR_TCPIP_HISLIP_VERSION: u32 = 1073677057;
pub const VI_ATTR_TCPIP_HISLIP_MAX_MESSAGE_KB: u32 = 1073677058;
pub const VI_ATTR_TCPIP_IS_HISLIP: u32 = 1073677059;
pub const VI_ATTR_TCPIP_HISLIP_ENCRYPTION_EN: u32 = 1073677060;
pub const VI_ATTR_JOB_ID: u32 = 1073692678;
pub const VI_ATTR_EVENT_TYPE: u32 = 1073692688;
pub const VI_ATTR_SIGP_STATUS_ID: u32 = 1073692689;
pub const VI_ATTR_RECV_TRIG_ID: u32 = 1073692690;
pub const VI_ATTR_INTR_STATUS_ID: u32 = 1073692707;
pub const VI_ATTR_STATUS: u32 = 1073692709;
pub const VI_ATTR_RET_COUNT_32: u32 = 1073692710;
pub const VI_ATTR_BUFFER: u32 = 1073692711;
pub const VI_ATTR_RECV_INTR_LEVEL: u32 = 1073692737;
pub const VI_ATTR_OPER_NAME: u32 = 3221176386;
pub const VI_ATTR_GPIB_RECV_CIC_STATE: u32 = 1073693075;
pub const VI_ATTR_RECV_TCPIP_ADDR: u32 = 3221176728;
pub const VI_ATTR_USB_RECV_INTR_SIZE: u32 = 1073693104;
pub const VI_ATTR_USB_RECV_INTR_DATA: u32 = 3221176753;
pub const VI_ATTR_PXI_RECV_INTR_SEQ: u32 = 1073693248;
pub const VI_ATTR_PXI_RECV_INTR_DATA: u32 = 1073693249;
pub const VI_ATTR_USER_DATA_64: u32 = 1073676298;
pub const VI_ATTR_RET_COUNT_64: u32 = 1073692712;
//pub const VI_ATTR_USER_DATA: u32 = 1073676298;
//pub const VI_ATTR_RET_COUNT: u32 = 1073692712;
pub const VI_ATTR_WIN_BASE_ADDR_64: u32 = 1073676443;
pub const VI_ATTR_WIN_SIZE_64: u32 = 1073676444;
pub const VI_ATTR_MEM_BASE_64: u32 = 1073676496;
pub const VI_ATTR_MEM_SIZE_64: u32 = 1073676497;
//pub const VI_ATTR_WIN_BASE_ADDR: u32 = 1073676443;
//pub const VI_ATTR_WIN_SIZE: u32 = 1073676444;
//pub const VI_ATTR_MEM_BASE: u32 = 1073676496;
//pub const VI_ATTR_MEM_SIZE: u32 = 1073676497;
//pub const VI_ATTR_PXI_MEM_BASE_BAR0: u32 = 1073676840;
//pub const VI_ATTR_PXI_MEM_BASE_BAR1: u32 = 1073676841;
//pub const VI_ATTR_PXI_MEM_BASE_BAR2: u32 = 1073676842;
//pub const VI_ATTR_PXI_MEM_BASE_BAR3: u32 = 1073676843;
//pub const VI_ATTR_PXI_MEM_BASE_BAR4: u32 = 1073676844;
//pub const VI_ATTR_PXI_MEM_BASE_BAR5: u32 = 1073676845;
//pub const VI_ATTR_PXI_MEM_SIZE_BAR0: u32 = 1073676856;
//pub const VI_ATTR_PXI_MEM_SIZE_BAR1: u32 = 1073676857;
//pub const VI_ATTR_PXI_MEM_SIZE_BAR2: u32 = 1073676858;
//pub const VI_ATTR_PXI_MEM_SIZE_BAR3: u32 = 1073676859;
//pub const VI_ATTR_PXI_MEM_SIZE_BAR4: u32 = 1073676860;
//pub const VI_ATTR_PXI_MEM_SIZE_BAR5: u32 = 1073676861;
pub const VI_EVENT_IO_COMPLETION: u32 = 1073684489;
pub const VI_EVENT_TRIG: u32 = 3221168138;
pub const VI_EVENT_SERVICE_REQ: u32 = 1073684491;
pub const VI_EVENT_CLEAR: u32 = 1073684493;
pub const VI_EVENT_EXCEPTION: u32 = 3221168142;
pub const VI_EVENT_GPIB_CIC: u32 = 1073684498;
pub const VI_EVENT_GPIB_TALK: u32 = 1073684499;
pub const VI_EVENT_GPIB_LISTEN: u32 = 1073684500;
pub const VI_EVENT_VXI_VME_SYSFAIL: u32 = 1073684509;
pub const VI_EVENT_VXI_VME_SYSRESET: u32 = 1073684510;
pub const VI_EVENT_VXI_SIGP: u32 = 1073684512;
pub const VI_EVENT_VXI_VME_INTR: u32 = 3221168161;
pub const VI_EVENT_PXI_INTR: u32 = 1073684514;
pub const VI_EVENT_TCPIP_CONNECT: u32 = 1073684534;
pub const VI_EVENT_USB_INTR: u32 = 1073684535;
pub const VI_ALL_ENABLED_EVENTS: u32 = 1073709055;
pub const VI_SUCCESS_EVENT_EN: u32 = 1073676290;
pub const VI_SUCCESS_EVENT_DIS: u32 = 1073676291;
pub const VI_SUCCESS_QUEUE_EMPTY: u32 = 1073676292;
pub const VI_SUCCESS_TERM_CHAR: u32 = 1073676293;
pub const VI_SUCCESS_MAX_CNT: u32 = 1073676294;
pub const VI_SUCCESS_DEV_NPRESENT: u32 = 1073676413;
pub const VI_SUCCESS_TRIG_MAPPED: u32 = 1073676414;
pub const VI_SUCCESS_QUEUE_NEMPTY: u32 = 1073676416;
pub const VI_SUCCESS_NCHAIN: u32 = 1073676440;
pub const VI_SUCCESS_NESTED_SHARED: u32 = 1073676441;
pub const VI_SUCCESS_NESTED_EXCLUSIVE: u32 = 1073676442;
pub const VI_SUCCESS_SYNC: u32 = 1073676443;
pub const VI_WARN_QUEUE_OVERFLOW: u32 = 1073676300;
pub const VI_WARN_CONFIG_NLOADED: u32 = 1073676407;
pub const VI_WARN_NULL_OBJECT: u32 = 1073676418;
pub const VI_WARN_NSUP_ATTR_STATE: u32 = 1073676420;
pub const VI_WARN_UNKNOWN_STATUS: u32 = 1073676421;
pub const VI_WARN_NSUP_BUF: u32 = 1073676424;
pub const VI_WARN_EXT_FUNC_NIMPL: u32 = 1073676457;
pub const VI_WARN_SERVER_CERT_UNTRUSTED: u32 = 1073676528;
pub const VI_ERROR_SYSTEM_ERROR: i32 = -1073807360;
pub const VI_ERROR_INV_OBJECT: i32 = -1073807346;
pub const VI_ERROR_RSRC_LOCKED: i32 = -1073807345;
pub const VI_ERROR_INV_EXPR: i32 = -1073807344;
pub const VI_ERROR_RSRC_NFOUND: i32 = -1073807343;
pub const VI_ERROR_INV_RSRC_NAME: i32 = -1073807342;
pub const VI_ERROR_INV_ACC_MODE: i32 = -1073807341;
pub const VI_ERROR_TMO: i32 = -1073807339;
pub const VI_ERROR_CLOSING_FAILED: i32 = -1073807338;
pub const VI_ERROR_INV_DEGREE: i32 = -1073807333;
pub const VI_ERROR_INV_JOB_ID: i32 = -1073807332;
pub const VI_ERROR_NSUP_ATTR: i32 = -1073807331;
pub const VI_ERROR_NSUP_ATTR_STATE: i32 = -1073807330;
pub const VI_ERROR_ATTR_READONLY: i32 = -1073807329;
pub const VI_ERROR_INV_LOCK_TYPE: i32 = -1073807328;
pub const VI_ERROR_INV_ACCESS_KEY: i32 = -1073807327;
pub const VI_ERROR_INV_EVENT: i32 = -1073807322;
pub const VI_ERROR_INV_MECH: i32 = -1073807321;
pub const VI_ERROR_HNDLR_NINSTALLED: i32 = -1073807320;
pub const VI_ERROR_INV_HNDLR_REF: i32 = -1073807319;
pub const VI_ERROR_INV_CONTEXT: i32 = -1073807318;
pub const VI_ERROR_NENABLED: i32 = -1073807313;
pub const VI_ERROR_ABORT: i32 = -1073807312;
pub const VI_ERROR_RAW_WR_PROT_VIOL: i32 = -1073807308;
pub const VI_ERROR_RAW_RD_PROT_VIOL: i32 = -1073807307;
pub const VI_ERROR_OUTP_PROT_VIOL: i32 = -1073807306;
pub const VI_ERROR_INP_PROT_VIOL: i32 = -1073807305;
pub const VI_ERROR_BERR: i32 = -1073807304;
pub const VI_ERROR_IN_PROGRESS: i32 = -1073807303;
pub const VI_ERROR_INV_SETUP: i32 = -1073807302;
pub const VI_ERROR_QUEUE_ERROR: i32 = -1073807301;
pub const VI_ERROR_ALLOC: i32 = -1073807300;
pub const VI_ERROR_INV_MASK: i32 = -1073807299;
pub const VI_ERROR_IO: i32 = -1073807298;
pub const VI_ERROR_INV_FMT: i32 = -1073807297;
pub const VI_ERROR_NSUP_FMT: i32 = -1073807295;
pub const VI_ERROR_LINE_IN_USE: i32 = -1073807294;
pub const VI_ERROR_LINE_NRESERVED: i32 = -1073807293;
pub const VI_ERROR_NSUP_MODE: i32 = -1073807290;
pub const VI_ERROR_SRQ_NOCCURRED: i32 = -1073807286;
pub const VI_ERROR_INV_SPACE: i32 = -1073807282;
pub const VI_ERROR_INV_OFFSET: i32 = -1073807279;
pub const VI_ERROR_INV_WIDTH: i32 = -1073807278;
pub const VI_ERROR_NSUP_OFFSET: i32 = -1073807276;
pub const VI_ERROR_NSUP_VAR_WIDTH: i32 = -1073807275;
pub const VI_ERROR_WINDOW_NMAPPED: i32 = -1073807273;
pub const VI_ERROR_RESP_PENDING: i32 = -1073807271;
pub const VI_ERROR_NLISTENERS: i32 = -1073807265;
pub const VI_ERROR_NCIC: i32 = -1073807264;
pub const VI_ERROR_NSYS_CNTLR: i32 = -1073807263;
pub const VI_ERROR_NSUP_OPER: i32 = -1073807257;
pub const VI_ERROR_INTR_PENDING: i32 = -1073807256;
pub const VI_ERROR_ASRL_PARITY: i32 = -1073807254;
pub const VI_ERROR_ASRL_FRAMING: i32 = -1073807253;
pub const VI_ERROR_ASRL_OVERRUN: i32 = -1073807252;
pub const VI_ERROR_TRIG_NMAPPED: i32 = -1073807250;
pub const VI_ERROR_NSUP_ALIGN_OFFSET: i32 = -1073807248;
pub const VI_ERROR_USER_BUF: i32 = -1073807247;
pub const VI_ERROR_RSRC_BUSY: i32 = -1073807246;
pub const VI_ERROR_NSUP_WIDTH: i32 = -1073807242;
pub const VI_ERROR_INV_PARAMETER: i32 = -1073807240;
pub const VI_ERROR_INV_PROT: i32 = -1073807239;
pub const VI_ERROR_INV_SIZE: i32 = -1073807237;
pub const VI_ERROR_WINDOW_MAPPED: i32 = -1073807232;
pub const VI_ERROR_NIMPL_OPER: i32 = -1073807231;
pub const VI_ERROR_INV_LENGTH: i32 = -1073807229;
pub const VI_ERROR_INV_MODE: i32 = -1073807215;
pub const VI_ERROR_SESN_NLOCKED: i32 = -1073807204;
pub const VI_ERROR_MEM_NSHARED: i32 = -1073807203;
pub const VI_ERROR_LIBRARY_NFOUND: i32 = -1073807202;
pub const VI_ERROR_NSUP_INTR: i32 = -1073807201;
pub const VI_ERROR_INV_LINE: i32 = -1073807200;
pub const VI_ERROR_FILE_ACCESS: i32 = -1073807199;
pub const VI_ERROR_FILE_IO: i32 = -1073807198;
pub const VI_ERROR_NSUP_LINE: i32 = -1073807197;
pub const VI_ERROR_NSUP_MECH: i32 = -1073807196;
pub const VI_ERROR_INTF_NUM_NCONFIG: i32 = -1073807195;
pub const VI_ERROR_CONN_LOST: i32 = -1073807194;
pub const VI_ERROR_NPERMISSION: i32 = -1073807192;
pub const VI_ERROR_SERVER_CERT: i32 = -1073807184;
pub const VI_FIND_BUFLEN: u32 = 256;
pub const VI_INTF_GPIB: u32 = 1;
pub const VI_INTF_VXI: u32 = 2;
pub const VI_INTF_GPIB_VXI: u32 = 3;
pub const VI_INTF_ASRL: u32 = 4;
pub const VI_INTF_PXI: u32 = 5;
pub const VI_INTF_TCPIP: u32 = 6;
pub const VI_INTF_USB: u32 = 7;
pub const VI_PROT_NORMAL: u32 = 1;
pub const VI_PROT_FDC: u32 = 2;
pub const VI_PROT_HS488: u32 = 3;
pub const VI_PROT_4882_STRS: u32 = 4;
pub const VI_PROT_USBTMC_VENDOR: u32 = 5;
pub const VI_FDC_NORMAL: u32 = 1;
pub const VI_FDC_STREAM: u32 = 2;
pub const VI_LOCAL_SPACE: u32 = 0;
pub const VI_A16_SPACE: u32 = 1;
pub const VI_A24_SPACE: u32 = 2;
pub const VI_A32_SPACE: u32 = 3;
pub const VI_A64_SPACE: u32 = 4;
pub const VI_PXI_ALLOC_SPACE: u32 = 9;
pub const VI_PXI_CFG_SPACE: u32 = 10;
pub const VI_PXI_BAR0_SPACE: u32 = 11;
pub const VI_PXI_BAR1_SPACE: u32 = 12;
pub const VI_PXI_BAR2_SPACE: u32 = 13;
pub const VI_PXI_BAR3_SPACE: u32 = 14;
pub const VI_PXI_BAR4_SPACE: u32 = 15;
pub const VI_PXI_BAR5_SPACE: u32 = 16;
pub const VI_OPAQUE_SPACE: u32 = 65535;
pub const VI_UNKNOWN_LA: i32 = -1;
pub const VI_UNKNOWN_SLOT: i32 = -1;
pub const VI_UNKNOWN_LEVEL: i32 = -1;
pub const VI_UNKNOWN_CHASSIS: i32 = -1;
pub const VI_QUEUE: u32 = 1;
pub const VI_HNDLR: u32 = 2;
pub const VI_SUSPEND_HNDLR: u32 = 4;
pub const VI_ALL_MECH: u32 = 65535;
pub const VI_ANY_HNDLR: u32 = 0;
pub const VI_TRIG_ALL: i32 = -2;
pub const VI_TRIG_SW: i32 = -1;
pub const VI_TRIG_TTL0: u32 = 0;
pub const VI_TRIG_TTL1: u32 = 1;
pub const VI_TRIG_TTL2: u32 = 2;
pub const VI_TRIG_TTL3: u32 = 3;
pub const VI_TRIG_TTL4: u32 = 4;
pub const VI_TRIG_TTL5: u32 = 5;
pub const VI_TRIG_TTL6: u32 = 6;
pub const VI_TRIG_TTL7: u32 = 7;
pub const VI_TRIG_ECL0: u32 = 8;
pub const VI_TRIG_ECL1: u32 = 9;
pub const VI_TRIG_ECL2: u32 = 10;
pub const VI_TRIG_ECL3: u32 = 11;
pub const VI_TRIG_ECL4: u32 = 12;
pub const VI_TRIG_ECL5: u32 = 13;
pub const VI_TRIG_STAR_SLOT1: u32 = 14;
pub const VI_TRIG_STAR_SLOT2: u32 = 15;
pub const VI_TRIG_STAR_SLOT3: u32 = 16;
pub const VI_TRIG_STAR_SLOT4: u32 = 17;
pub const VI_TRIG_STAR_SLOT5: u32 = 18;
pub const VI_TRIG_STAR_SLOT6: u32 = 19;
pub const VI_TRIG_STAR_SLOT7: u32 = 20;
pub const VI_TRIG_STAR_SLOT8: u32 = 21;
pub const VI_TRIG_STAR_SLOT9: u32 = 22;
pub const VI_TRIG_STAR_SLOT10: u32 = 23;
pub const VI_TRIG_STAR_SLOT11: u32 = 24;
pub const VI_TRIG_STAR_SLOT12: u32 = 25;
pub const VI_TRIG_STAR_INSTR: u32 = 26;
pub const VI_TRIG_PANEL_IN: u32 = 27;
pub const VI_TRIG_PANEL_OUT: u32 = 28;
pub const VI_TRIG_STAR_VXI0: u32 = 29;
pub const VI_TRIG_STAR_VXI1: u32 = 30;
pub const VI_TRIG_STAR_VXI2: u32 = 31;
pub const VI_TRIG_TTL8: u32 = 32;
pub const VI_TRIG_TTL9: u32 = 33;
pub const VI_TRIG_TTL10: u32 = 34;
pub const VI_TRIG_TTL11: u32 = 35;
pub const VI_TRIG_PROT_DEFAULT: u32 = 0;
pub const VI_TRIG_PROT_ON: u32 = 1;
pub const VI_TRIG_PROT_OFF: u32 = 2;
pub const VI_TRIG_PROT_SYNC: u32 = 5;
pub const VI_TRIG_PROT_RESERVE: u32 = 6;
pub const VI_TRIG_PROT_UNRESERVE: u32 = 7;
pub const VI_READ_BUF: u32 = 1;
pub const VI_WRITE_BUF: u32 = 2;
pub const VI_READ_BUF_DISCARD: u32 = 4;
pub const VI_WRITE_BUF_DISCARD: u32 = 8;
pub const VI_IO_IN_BUF: u32 = 16;
pub const VI_IO_OUT_BUF: u32 = 32;
pub const VI_IO_IN_BUF_DISCARD: u32 = 64;
pub const VI_IO_OUT_BUF_DISCARD: u32 = 128;
pub const VI_FLUSH_ON_ACCESS: u32 = 1;
pub const VI_FLUSH_WHEN_FULL: u32 = 2;
pub const VI_FLUSH_DISABLE: u32 = 3;
pub const VI_NMAPPED: u32 = 1;
pub const VI_USE_OPERS: u32 = 2;
pub const VI_DEREF_ADDR: u32 = 3;
pub const VI_TMO_IMMEDIATE: u32 = 0;
pub const VI_TMO_INFINITE: u32 = 4294967295;
pub const VI_NO_LOCK: u32 = 0;
pub const VI_EXCLUSIVE_LOCK: u32 = 1;
pub const VI_SHARED_LOCK: u32 = 2;
pub const VI_LOAD_CONFIG: u32 = 4;
pub const VI_NO_SEC_ADDR: u32 = 65535;
pub const VI_ASRL_PAR_NONE: u32 = 0;
pub const VI_ASRL_PAR_ODD: u32 = 1;
pub const VI_ASRL_PAR_EVEN: u32 = 2;
pub const VI_ASRL_PAR_MARK: u32 = 3;
pub const VI_ASRL_PAR_SPACE: u32 = 4;
pub const VI_ASRL_STOP_ONE: u32 = 10;
pub const VI_ASRL_STOP_ONE5: u32 = 15;
pub const VI_ASRL_STOP_TWO: u32 = 20;
pub const VI_ASRL_FLOW_NONE: u32 = 0;
pub const VI_ASRL_FLOW_XON_XOFF: u32 = 1;
pub const VI_ASRL_FLOW_RTS_CTS: u32 = 2;
pub const VI_ASRL_FLOW_DTR_DSR: u32 = 4;
pub const VI_ASRL_END_NONE: u32 = 0;
pub const VI_ASRL_END_LAST_BIT: u32 = 1;
pub const VI_ASRL_END_TERMCHAR: u32 = 2;
pub const VI_ASRL_END_BREAK: u32 = 3;
pub const VI_STATE_ASSERTED: u32 = 1;
pub const VI_STATE_UNASSERTED: u32 = 0;
pub const VI_STATE_UNKNOWN: i32 = -1;
pub const VI_BIG_ENDIAN: u32 = 0;
pub const VI_LITTLE_ENDIAN: u32 = 1;
pub const VI_DATA_PRIV: u32 = 0;
pub const VI_DATA_NPRIV: u32 = 1;
pub const VI_PROG_PRIV: u32 = 2;
pub const VI_PROG_NPRIV: u32 = 3;
pub const VI_BLCK_PRIV: u32 = 4;
pub const VI_BLCK_NPRIV: u32 = 5;
pub const VI_D64_PRIV: u32 = 6;
pub const VI_D64_NPRIV: u32 = 7;
pub const VI_D64_2EVME: u32 = 8;
pub const VI_D64_SST160: u32 = 9;
pub const VI_D64_SST267: u32 = 10;
pub const VI_D64_SST320: u32 = 11;
pub const VI_WIDTH_8: u32 = 1;
pub const VI_WIDTH_16: u32 = 2;
pub const VI_WIDTH_32: u32 = 4;
pub const VI_WIDTH_64: u32 = 8;
pub const VI_GPIB_REN_DEASSERT: u32 = 0;
pub const VI_GPIB_REN_ASSERT: u32 = 1;
pub const VI_GPIB_REN_DEASSERT_GTL: u32 = 2;
pub const VI_GPIB_REN_ASSERT_ADDRESS: u32 = 3;
pub const VI_GPIB_REN_ASSERT_LLO: u32 = 4;
pub const VI_GPIB_REN_ASSERT_ADDRESS_LLO: u32 = 5;
pub const VI_GPIB_REN_ADDRESS_GTL: u32 = 6;
pub const VI_GPIB_ATN_DEASSERT: u32 = 0;
pub const VI_GPIB_ATN_ASSERT: u32 = 1;
pub const VI_GPIB_ATN_DEASSERT_HANDSHAKE: u32 = 2;
pub const VI_GPIB_ATN_ASSERT_IMMEDIATE: u32 = 3;
pub const VI_GPIB_HS488_DISABLED: u32 = 0;
pub const VI_GPIB_HS488_NIMPL: i32 = -1;
pub const VI_GPIB_UNADDRESSED: u32 = 0;
pub const VI_GPIB_TALKER: u32 = 1;
pub const VI_GPIB_LISTENER: u32 = 2;
pub const VI_VXI_CMD16: u32 = 512;
pub const VI_VXI_CMD16_RESP16: u32 = 514;
pub const VI_VXI_RESP16: u32 = 2;
pub const VI_VXI_CMD32: u32 = 1024;
pub const VI_VXI_CMD32_RESP16: u32 = 1026;
pub const VI_VXI_CMD32_RESP32: u32 = 1028;
pub const VI_VXI_RESP32: u32 = 4;
pub const VI_ASSERT_SIGNAL: i32 = -1;
pub const VI_ASSERT_USE_ASSIGNED: u32 = 0;
pub const VI_ASSERT_IRQ1: u32 = 1;
pub const VI_ASSERT_IRQ2: u32 = 2;
pub const VI_ASSERT_IRQ3: u32 = 3;
pub const VI_ASSERT_IRQ4: u32 = 4;
pub const VI_ASSERT_IRQ5: u32 = 5;
pub const VI_ASSERT_IRQ6: u32 = 6;
pub const VI_ASSERT_IRQ7: u32 = 7;
pub const VI_UTIL_ASSERT_SYSRESET: u32 = 1;
pub const VI_UTIL_ASSERT_SYSFAIL: u32 = 2;
pub const VI_UTIL_DEASSERT_SYSFAIL: u32 = 3;
pub const VI_VXI_CLASS_MEMORY: u32 = 0;
pub const VI_VXI_CLASS_EXTENDED: u32 = 1;
pub const VI_VXI_CLASS_MESSAGE: u32 = 2;
pub const VI_VXI_CLASS_REGISTER: u32 = 3;
pub const VI_VXI_CLASS_OTHER: u32 = 4;
pub const VI_PXI_ADDR_NONE: u32 = 0;
pub const VI_PXI_ADDR_MEM: u32 = 1;
pub const VI_PXI_ADDR_IO: u32 = 2;
pub const VI_PXI_ADDR_CFG: u32 = 3;
pub const VI_TRIG_UNKNOWN: i32 = -1;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_0: u32 = 1000;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_1: u32 = 1001;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_2: u32 = 1002;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_3: u32 = 1003;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_4: u32 = 1004;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_5: u32 = 1005;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_6: u32 = 1006;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_7: u32 = 1007;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_8: u32 = 1008;
pub const VI_PXI_LBUS_STAR_TRIG_BUS_9: u32 = 1009;
pub const VI_PXI_STAR_TRIG_CONTROLLER: u32 = 1413;
pub const VI_ERROR_INV_SESSION: i32 = -1073807346;
pub const VI_INFINITE: u32 = 4294967295;
pub const VI_NORMAL: u32 = 1;
pub const VI_FDC: u32 = 2;
pub const VI_HS488: u32 = 3;
pub const VI_ASRL488: u32 = 4;
pub const VI_ASRL_IN_BUF: u32 = 16;
pub const VI_ASRL_OUT_BUF: u32 = 32;
pub const VI_ASRL_IN_BUF_DISCARD: u32 = 64;
pub const VI_ASRL_OUT_BUF_DISCARD: u32 = 128;

pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;

macro_rules! c_macros_to_rust_consts {
    ($(#define $name:ident ($value:ident))*) => {
        $(pub const $name: u32 = $value;)*
    };
}

#[cfg(target_pointer_width = "64")]
c_macros_to_rust_consts! {
    #define VI_ATTR_USER_DATA (VI_ATTR_USER_DATA_64)
    #define VI_ATTR_RET_COUNT (VI_ATTR_RET_COUNT_64)
    #define VI_ATTR_WIN_BASE_ADDR (VI_ATTR_WIN_BASE_ADDR_64)
    #define VI_ATTR_WIN_SIZE (VI_ATTR_WIN_SIZE_64)
    #define VI_ATTR_MEM_BASE (VI_ATTR_MEM_BASE_64)
    #define VI_ATTR_MEM_SIZE (VI_ATTR_MEM_SIZE_64)
    #define VI_ATTR_PXI_MEM_BASE_BAR0 (VI_ATTR_PXI_MEM_BASE_BAR0_64)
    #define VI_ATTR_PXI_MEM_BASE_BAR1 (VI_ATTR_PXI_MEM_BASE_BAR1_64)
    #define VI_ATTR_PXI_MEM_BASE_BAR2 (VI_ATTR_PXI_MEM_BASE_BAR2_64)
    #define VI_ATTR_PXI_MEM_BASE_BAR3 (VI_ATTR_PXI_MEM_BASE_BAR3_64)
    #define VI_ATTR_PXI_MEM_BASE_BAR4 (VI_ATTR_PXI_MEM_BASE_BAR4_64)
    #define VI_ATTR_PXI_MEM_BASE_BAR5 (VI_ATTR_PXI_MEM_BASE_BAR5_64)
    #define VI_ATTR_PXI_MEM_SIZE_BAR0 (VI_ATTR_PXI_MEM_SIZE_BAR0_64)
    #define VI_ATTR_PXI_MEM_SIZE_BAR1 (VI_ATTR_PXI_MEM_SIZE_BAR1_64)
    #define VI_ATTR_PXI_MEM_SIZE_BAR2 (VI_ATTR_PXI_MEM_SIZE_BAR2_64)
    #define VI_ATTR_PXI_MEM_SIZE_BAR3 (VI_ATTR_PXI_MEM_SIZE_BAR3_64)
    #define VI_ATTR_PXI_MEM_SIZE_BAR4 (VI_ATTR_PXI_MEM_SIZE_BAR4_64)
    #define VI_ATTR_PXI_MEM_SIZE_BAR5 (VI_ATTR_PXI_MEM_SIZE_BAR5_64)
}

#[cfg(not(target_pointer_width = "64"))]
c_macros_to_rust_consts! {
    #define VI_ATTR_USER_DATA (VI_ATTR_USER_DATA_32)
    #define VI_ATTR_RET_COUNT (VI_ATTR_RET_COUNT_32)
    #define VI_ATTR_WIN_BASE_ADDR (VI_ATTR_WIN_BASE_ADDR_32)
    #define VI_ATTR_WIN_SIZE (VI_ATTR_WIN_SIZE_32)
    #define VI_ATTR_MEM_BASE (VI_ATTR_MEM_BASE_32)
    #define VI_ATTR_MEM_SIZE (VI_ATTR_MEM_SIZE_32)
    #define VI_ATTR_PXI_MEM_BASE_BAR0 (VI_ATTR_PXI_MEM_BASE_BAR0_32)
    #define VI_ATTR_PXI_MEM_BASE_BAR1 (VI_ATTR_PXI_MEM_BASE_BAR1_32)
    #define VI_ATTR_PXI_MEM_BASE_BAR2 (VI_ATTR_PXI_MEM_BASE_BAR2_32)
    #define VI_ATTR_PXI_MEM_BASE_BAR3 (VI_ATTR_PXI_MEM_BASE_BAR3_32)
    #define VI_ATTR_PXI_MEM_BASE_BAR4 (VI_ATTR_PXI_MEM_BASE_BAR4_32)
    #define VI_ATTR_PXI_MEM_BASE_BAR5 (VI_ATTR_PXI_MEM_BASE_BAR5_32)
    #define VI_ATTR_PXI_MEM_SIZE_BAR0 (VI_ATTR_PXI_MEM_SIZE_BAR0_32)
    #define VI_ATTR_PXI_MEM_SIZE_BAR1 (VI_ATTR_PXI_MEM_SIZE_BAR1_32)
    #define VI_ATTR_PXI_MEM_SIZE_BAR2 (VI_ATTR_PXI_MEM_SIZE_BAR2_32)
    #define VI_ATTR_PXI_MEM_SIZE_BAR3 (VI_ATTR_PXI_MEM_SIZE_BAR3_32)
    #define VI_ATTR_PXI_MEM_SIZE_BAR4 (VI_ATTR_PXI_MEM_SIZE_BAR4_32)
    #define VI_ATTR_PXI_MEM_SIZE_BAR5 (VI_ATTR_PXI_MEM_SIZE_BAR5_32)
}

/// A UInt that is the same size as the target's pointer width
// !!!
#[cfg(target_pointer_width = "64")]
pub type ViUIntPtrSize = ViUInt64;
#[cfg(not(target_pointer_width = "64"))]
pub type ViUIntPtrSize = ViUInt32;
// !!!

/*Type definitions */
pub type ViUInt64 = u64;
pub type ViInt64 = i64;
pub type ViPUInt64 = *mut ViUInt64;
pub type ViAUInt64 = *mut ViUInt64;
pub type ViPInt64 = *mut ViInt64;
pub type ViAInt64 = *mut ViInt64;
pub type ViUInt32 = u32;
pub type ViPUInt32 = *mut ViUInt32;
pub type ViAUInt32 = *mut ViUInt32;
pub type ViInt32 = i32;
pub type ViPInt32 = *mut ViInt32;
pub type ViAInt32 = *mut ViInt32;
pub type ViUInt16 = u16;
pub type ViPUInt16 = *mut ViUInt16;
pub type ViAUInt16 = *mut ViUInt16;
pub type ViInt16 = i16;
pub type ViPInt16 = *mut ViInt16;
pub type ViAInt16 = *mut ViInt16;
pub type ViUInt8 = ::std::os::raw::c_uchar;
pub type ViPUInt8 = *mut ViUInt8;
pub type ViAUInt8 = *mut ViUInt8;
pub type ViInt8 = ::std::os::raw::c_schar;
pub type ViPInt8 = *mut ViInt8;
pub type ViAInt8 = *mut ViInt8;
pub type ViChar = ::std::os::raw::c_char;
pub type ViPChar = *mut ViChar;
pub type ViAChar = *mut ViChar;
pub type ViByte = ::std::os::raw::c_uchar;
pub type ViPByte = *mut ViByte;
pub type ViAByte = *mut ViByte;
pub type ViAddr = *mut ::std::os::raw::c_void;
pub type ViPAddr = *mut ViAddr;
pub type ViAAddr = *mut ViAddr;
pub type ViReal32 = f32;
pub type ViPReal32 = *mut ViReal32;
pub type ViAReal32 = *mut ViReal32;
pub type ViReal64 = f64;
pub type ViPReal64 = *mut ViReal64;
pub type ViAReal64 = *mut ViReal64;
pub type ViBuf = ViPByte;
pub type ViConstBuf = *const ViByte;
pub type ViPBuf = ViPByte;
pub type ViABuf = *mut ViPByte;
pub type ViString = ViPChar;
pub type ViConstString = *const ViChar;
pub type ViPString = ViPChar;
pub type ViAString = *mut ViPChar;
pub type ViRsrc = ViString;
pub type ViConstRsrc = ViConstString;
pub type ViPRsrc = ViString;
pub type ViARsrc = *mut ViString;
pub type ViBoolean = ViUInt16;
pub type ViPBoolean = *mut ViBoolean;
pub type ViABoolean = *mut ViBoolean;
pub type ViStatus = ViInt32;
pub type ViPStatus = *mut ViStatus;
pub type ViAStatus = *mut ViStatus;
pub type ViVersion = ViUInt32;
pub type ViPVersion = *mut ViVersion;
pub type ViAVersion = *mut ViVersion;
pub type ViObject = ViUInt32;
pub type ViPObject = *mut ViObject;
pub type ViAObject = *mut ViObject;
pub type ViSession = ViObject;
pub type ViPSession = *mut ViSession;
pub type ViASession = *mut ViSession;
pub type ViAttr = ViUInt32;
pub type ViEvent = ViObject;
pub type ViPEvent = *mut ViEvent;
pub type ViFindList = ViObject;
pub type ViPFindList = *mut ViFindList;
// !!!
pub type ViBusAddress = ViUIntPtrSize;
pub type ViBusSize = ViUIntPtrSize;
pub type ViAttrState = ViUIntPtrSize;
// !!!
pub type ViBusAddress64 = ViUInt64;
pub type ViPBusAddress64 = *mut ViBusAddress64;
pub type ViEventType = ViUInt32;
pub type ViPEventType = *mut ViEventType;
pub type ViAEventType = *mut ViEventType;
pub type ViPAttrState = *mut ::std::os::raw::c_void;
pub type ViPAttr = *mut ViAttr;
pub type ViAAttr = *mut ViAttr;
pub type ViKeyId = ViString;
pub type ViConstKeyId = ViConstString;
pub type ViPKeyId = ViPString;
pub type ViJobId = ViUInt32;
pub type ViPJobId = *mut ViJobId;
pub type ViAccessMode = ViUInt32;
pub type ViPAccessMode = *mut ViAccessMode;
pub type ViPBusAddress = *mut ViBusAddress;
pub type ViEventFilter = ViUInt32;
pub type ViVAList = va_list;
pub type ViHndlr = ::std::option::Option<
    unsafe extern "system" fn(
        vi: ViSession,
        eventType: ViEventType,
        event: ViEvent,
        userHandle: ViAddr,
    ) -> ViStatus,
>;