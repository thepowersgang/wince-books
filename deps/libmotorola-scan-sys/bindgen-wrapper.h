//#include "windows.h"

#define FAR
#define WINAPI

// winbase.h
#define MAX_PATH	260

// windef.h
typedef int BOOL;
typedef BOOL *LPBOOL;
typedef unsigned char BYTE;
typedef unsigned short WORD;
typedef unsigned long DWORD;
typedef DWORD *LPDWORD;

// winnt.h
typedef char CHAR;
typedef short SHORT;
typedef unsigned short USHORT;
typedef unsigned int UINT;
typedef long LONG;
typedef unsigned long ULONG;
typedef void *LPVOID;
typedef const char* LPCSTR;

typedef unsigned short wchar_t;
typedef wchar_t WCHAR;
typedef WCHAR TCHAR;
typedef const TCHAR* LPCTSTR;

typedef void* HANDLE;
typedef HANDLE *PHANDLE,*LPHANDLE;

typedef struct nHWND { int i; }* HWND;

// windef.h
typedef struct tagRECT {
	LONG left;
	LONG top;
	LONG right;
	LONG bottom;
} RECT,*PRECT,*LPRECT;

// winbase.h
typedef struct _SYSTEMTIME {
	WORD wYear;
	WORD wMonth;
	WORD wDayOfWeek;
	WORD wDay;
	WORD wHour;
	WORD wMinute;
	WORD wSecond;
	WORD wMilliseconds;
} SYSTEMTIME,*LPSYSTEMTIME;

extern DWORD WINAPI GetLastError(void);

#include "../../ThirdParty/EMDK/Include/ArmV4/ScanCApi.h"
