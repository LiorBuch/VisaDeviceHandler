use std::convert::From;
use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

///This is a library-specific error that is returned by all calls to all APIs.
#[derive(Debug)]
pub enum VisaWrapperError {
    ///The library could not be opened.
    OpeningLibraryError(IoError),
    ///The symbol could not be obtained.
    SymbolGettingError(IoError),
    ///Value of the symbol was null.
    NullSymbol,
    ///Address could not be matched to a dynamic link library
    PathNotMatchingLibrary(IoError),
    ///Null Character in path name
    NullCharacter,
    ///Unsupported Target
    UnsupportedPlatform,
}

impl Display for VisaWrapperError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::VisaWrapperError::*;
        f.write_str(&format!("{:?}", self))?;
        match self {
            OpeningLibraryError(msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            }
            SymbolGettingError(msg) => {
                f.write_str(": ")?;
                msg.fmt(f)
            }
            NullSymbol => f.write_str(": Symbol is Null."),
            PathNotMatchingLibrary(msg) => {
                f.write_str(": Path does not lead to a library.")?;
                msg.fmt(f)
            }
            NullCharacter => f.write_str(": The path contains a null character."),
            UnsupportedPlatform => f.write_str(": The target system is not supported by visa."),
        }
    }
}

impl From<dlopen2::Error> for VisaWrapperError {
    fn from(value: dlopen2::Error) -> Self {
        match value {
            dlopen2::Error::NullCharacter(_) => VisaWrapperError::NullCharacter,
            dlopen2::Error::OpeningLibraryError(e) => VisaWrapperError::OpeningLibraryError(e),
            dlopen2::Error::SymbolGettingError(e) => VisaWrapperError::SymbolGettingError(e),
            dlopen2::Error::NullSymbol => VisaWrapperError::NullSymbol,
            dlopen2::Error::AddrNotMatchingDll(e) => VisaWrapperError::PathNotMatchingLibrary(e),
        }
    }
}

impl ErrorTrait for VisaWrapperError {
    fn description(&self) -> &str {
        use self::VisaWrapperError::*;
        match *self {
            OpeningLibraryError(_) => "Could not open library",
            SymbolGettingError(_) => "Could not obtain symbol from the library",
            NullSymbol => "The symbol is NULL",
            PathNotMatchingLibrary(_) => "Address does not match any dynamic link library",
            NullCharacter => "Uncategorized",
            UnsupportedPlatform => "The target system is not supported by visa",
        }
    }

    fn cause(&self) -> Option<&dyn ErrorTrait> {
        use self::VisaWrapperError::*;
        match self {
            &OpeningLibraryError(_)
            | &SymbolGettingError(_)
            | &NullSymbol
            | &PathNotMatchingLibrary(_)
            | &NullCharacter
            | &UnsupportedPlatform => None,
        }
    }
}