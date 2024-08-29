use mutex_logger::logger::{MLogger,Verbosity};
use visa::{VI_ERROR_ALLOC, VI_ERROR_ASRL_FRAMING, VI_ERROR_ASRL_OVERRUN, VI_ERROR_ASRL_PARITY, VI_ERROR_BERR, VI_ERROR_CONN_LOST, VI_ERROR_INP_PROT_VIOL, VI_ERROR_INTF_NUM_NCONFIG, VI_ERROR_INV_ACC_MODE, VI_ERROR_INV_EXPR, VI_ERROR_INV_OBJECT, VI_ERROR_INV_RSRC_NAME, VI_ERROR_INV_SETUP, VI_ERROR_IO, VI_ERROR_LIBRARY_NFOUND, VI_ERROR_NCIC, VI_ERROR_NLISTENERS, VI_ERROR_NPERMISSION, VI_ERROR_NSUP_OPER, VI_ERROR_OUTP_PROT_VIOL, VI_ERROR_RAW_RD_PROT_VIOL, VI_ERROR_RAW_WR_PROT_VIOL, VI_ERROR_RSRC_BUSY, VI_ERROR_RSRC_LOCKED, VI_ERROR_RSRC_NFOUND, VI_ERROR_SYSTEM_ERROR, VI_ERROR_TMO, VI_SUCCESS, VI_SUCCESS_DEV_NPRESENT, VI_SUCCESS_MAX_CNT, VI_SUCCESS_TERM_CHAR, VI_WARN_CONFIG_NLOADED};

pub fn test_open_rm_status(status: i32) -> Result<(), String> {
    match status as u32 {
        VI_SUCCESS => return Ok(()),
        VI_WARN_CONFIG_NLOADED => {
            println!("Warning, ni-visa config not loaded");
        }
        _ => {
            return Err(test_error_code_for_openrm(status));
        }
    }
    Ok(())
}
pub fn test_write_status(status: i32,logger:&MLogger) -> Result<(), String> {
    match status as u32 {
        VI_SUCCESS => return Ok(()),
        _ => return{
            let error = test_error_code_for_write(status);
            logger.log(&error, Verbosity::Error)?;
            Err(error)
        },
    }
}
pub fn test_read_status(status: i32,logger:&MLogger) -> Result<(), String> {
    match status as u32 {
        VI_SUCCESS => return Ok(()),
        VI_SUCCESS_TERM_CHAR => {
            logger.log("Warning, The specified termination character was read but no END indicator was received. This completion code is returned regardless of whether the number of bytes read is equal to count.", Verbosity::Warn)?;
        },
        VI_SUCCESS_MAX_CNT => {
            logger.log("Warning, The number of bytes read is equal to count. No END indicator was received and no termination character was read.", Verbosity::Warn)?;
        },
        _ => return{
            let error = test_error_code_for_read(status);
            logger.log(&error, Verbosity::Error)?;
            Err(error)
        },
    }
    Ok(())
}
pub fn test_viopen_status(status: i32,logger:&MLogger) -> Result<(), String> {
    match status as u32 {
        VI_SUCCESS => return Ok(()),
        VI_SUCCESS_DEV_NPRESENT => {
            logger.log("Session opened successfully, but the device at the specified address is not responding.", Verbosity::Warn)?;
        },
        VI_WARN_CONFIG_NLOADED => {
            logger.log("The specified configuration either does not exist or could not be loaded; using VISA-specified defaults.", Verbosity::Warn)?;
        },
        _ => return{
            let error = test_error_code_for_viopen(status);
            logger.log(&error, Verbosity::Error)?;
            Err(error)
        },
    }
    Ok(())
}
pub fn test_find_rsc_status(status: i32,logger:&MLogger) -> Result<(), String> {
    match status as u32 {
        VI_SUCCESS => return Ok(()),
        _ => return{
            let error = test_error_code_for_find_rsc(status);
            logger.log(&error, Verbosity::Error)?;
            Err(error)
        },
    }
}
fn test_error_code_for_openrm(status: i32) -> String {
    match status {
        // OpenDefaultRM
        VI_ERROR_SYSTEM_ERROR => {
            return format_code_string(status, "The VISA system failed to initialize.");
        }
        VI_ERROR_ALLOC => {
            return format_code_string(status, "Insufficient system resources to create a session to the Default Resource Manager resource.");
        }
        VI_ERROR_INV_SETUP => {
            return format_code_string(status, "Some implementation-specific configuration file is corrupt or does not exist.");
        }
        VI_ERROR_LIBRARY_NFOUND => {
            return format_code_string(status, "A code library required by VISA could not be located or loaded.");
        }
        // Unregistered
        _ => return format!("Unregisterd error code:{status}"),
    }
}
fn test_error_code_for_write(status: i32) -> String {
    match status {
        // viWrite
        VI_ERROR_INV_OBJECT => {
            return format_code_string(status, "The given session reference is invalid.");
        }
        VI_ERROR_NSUP_OPER => {
            return format_code_string(status, "The given vi does not support this operation.");
        }
        VI_ERROR_RSRC_LOCKED => {
            return format_code_string(status, "Specified operation could not be performed because the resource identified by vi has been locked for this kind of access.");
        }
        VI_ERROR_TMO => {
            return format_code_string(status, "Timeout expired before operation completed.");
        }
        VI_ERROR_RAW_WR_PROT_VIOL => {
            return format_code_string(status, "Violation of raw write protocol occurred during transfer.");
        }
        VI_ERROR_RAW_RD_PROT_VIOL => {
            return format_code_string(status, "Violation of raw read protocol occurred during transfer.");
        }
        VI_ERROR_INP_PROT_VIOL => {
            return format_code_string(status, "Device reported an input protocol error during transfer.");
        }
        VI_ERROR_BERR => {
            return format_code_string(status, "Bus error occurred during transfer.");
        }
        VI_ERROR_INV_SETUP => {
            return format_code_string(status, "Unable to start write operation because setup is invalid (due to attributes being set to an inconsistent state).");
        }
        VI_ERROR_NLISTENERS => {
            return format_code_string(status, "No-listeners condition is detected (both NRFD and NDAC are unasserted).");
        }
        VI_ERROR_NCIC => {
            return format_code_string(status, "The interface associated with the given vi is not currently the controller in charge.");
        }
        VI_ERROR_IO => {
            return format_code_string(status, "An unknown I/O error occurred during transfer.");
        }
        VI_ERROR_CONN_LOST => {
            return format_code_string(status, "The I/O connection for the given session has been lost.");
        }
        // Unregistered
        _ => return format!("Unregisterd error code:{status}"),
    }
}
fn test_error_code_for_read(status: i32) -> String {
    match status {
        // viWrite
        VI_ERROR_INV_OBJECT => {
            return format_code_string(status, "The given session reference is invalid.");
        }
        VI_ERROR_NSUP_OPER => {
            return format_code_string(status, "The given vi does not support this operation.");
        }
        VI_ERROR_RSRC_LOCKED => {
            return format_code_string(status, "Specified operation could not be performed because the resource identified by vi has been locked for this kind of access.");
        }
        VI_ERROR_TMO => {
            return format_code_string(status, "Timeout expired before operation completed.");
        }
        VI_ERROR_RAW_WR_PROT_VIOL => {
            return format_code_string(status, "Violation of raw write protocol occurred during transfer.");
        }
        VI_ERROR_RAW_RD_PROT_VIOL => {
            return format_code_string(status, "Violation of raw read protocol occurred during transfer.");
        }
        VI_ERROR_OUTP_PROT_VIOL => {
            return format_code_string(status, "Device reported an output protocol error during transfer.");
        }
        VI_ERROR_BERR => {
            return format_code_string(status, "Bus error occurred during transfer.");
        }
        VI_ERROR_INV_SETUP => {
            return format_code_string(status, "Unable to start read operation because setup is invalid (due to attributes being set to an inconsistent state).");
        }
        VI_ERROR_NLISTENERS => {
            return format_code_string(status, "No-listeners condition is detected (both NRFD and NDAC are unasserted).");
        }
        VI_ERROR_NCIC => {
            return format_code_string(status, "The interface associated with the given vi is not currently the controller in charge.");
        }
        VI_ERROR_IO => {
            return format_code_string(status, "An unknown I/O error occurred during transfer.");
        }
        VI_ERROR_CONN_LOST => {
            return format_code_string(status, "The I/O connection for the given session has been lost.");
        }
        VI_ERROR_ASRL_PARITY => {
            return format_code_string(status, "A parity error occurred during transfer.");
        }
        VI_ERROR_ASRL_FRAMING => {
            return format_code_string(status, "A framing error occurred during transfer.");
        }
        VI_ERROR_ASRL_OVERRUN => {
            return format_code_string(status, "An overrun error occurred during transfer. A character was not read from the hardware before the next character arrived.");
        }
        // Unregistered
        _ => return format!("Unregisterd error code:{status}"),
    }
}
fn test_error_code_for_find_rsc(status: i32) -> String {
    match status {
        // viWrite
        VI_ERROR_INV_OBJECT => {
            return format_code_string(status, "The given session reference is invalid.");
        }
        VI_ERROR_NSUP_OPER => {
            return format_code_string(status, "The given sesn does not support this operation. This operation is supported only by a Resource Manager session.");
        }
        VI_ERROR_INV_EXPR => {
            return format_code_string(status, "Invalid expression specified for search.");
        }
        VI_ERROR_RSRC_NFOUND => {
            return format_code_string(status, "Specified expression does not match any devices.");
        }
        // Unregistered
        _ => return format!("Unregisterd error code:{status}"),
    }
}
fn test_error_code_for_viopen(status: i32) -> String {
    match status {
        // viWrite
        VI_ERROR_INV_OBJECT => {
            return format_code_string(status, "The given session reference is invalid.");
        }
        VI_ERROR_NSUP_OPER => {
            return format_code_string(status, "The given sesn does not support this operation. This operation is supported only by a Resource Manager session.");
        }
        VI_ERROR_INV_RSRC_NAME => {
            return format_code_string(status, "Invalid resource reference specified. Parsing error.");
        }
        VI_ERROR_INV_ACC_MODE => {
            return format_code_string(status, "Invalid access mode.");
        }
        VI_ERROR_RSRC_NFOUND => {
            return format_code_string(status, "Insufficient location information or resource not present in the system.");
        }
        VI_ERROR_ALLOC => {
            return format_code_string(status, "Insufficient system resources to open a session.");
        }
        VI_ERROR_RSRC_BUSY => {
            return format_code_string(status, "The resource is valid, but VISA cannot currently access it.");
        }
        VI_ERROR_RSRC_LOCKED => {
            return format_code_string(status, "Specified type of lock cannot be obtained because the resource is already locked with a lock type incompatible with the lock requested.");
        }
        VI_ERROR_TMO => {
            return format_code_string(status, "A session to the resource could not be obtained within the specified openTimeout period.");
        }
        VI_ERROR_LIBRARY_NFOUND => {
            return format_code_string(status, "A code library required by VISA could not be located or loaded.");
        }
        VI_ERROR_INTF_NUM_NCONFIG => {
            return format_code_string(status, "The interface type is valid, but the specified interface number is not configured.");
        }
        VI_ERROR_NPERMISSION => {
            return format_code_string(status, "Access to the remote machine is denied.");
        }
        // Unregistered
        _ => return format!("Unregisterd error code:{status}"),
    }
}

fn format_code_string(code: i32, string: &str) -> String {

    return format!("Visa Error Code:{code} -> {string}");
}