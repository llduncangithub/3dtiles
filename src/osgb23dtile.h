#pragma once

// The following ifdef block is the standard way of creating macros which make exporting
// from a DLL simpler. All files within this DLL are compiled with the PINVOKELIB_EXPORTS
// symbol defined on the command line. this symbol should not be defined on any project
// that uses this DLL. This way any other project whose source files include this file see
// PINVOKELIB_API functions as being imported from a DLL, wheras this DLL sees symbols
// defined with this macro as being exported.
#ifdef TILE_EXPORTS
#define TILE_API __declspec(dllexport)
#else
#define TILE_API __declspec(dllimport)
#endif

extern "C" TILE_API bool osgb23dtile_path(
	const char* in_path, const char* out_path,
	double *box, int max_lvl);

extern "C" TILE_API bool osgb23dtile(
	const char* in, const char* out);

extern "C" TILE_API bool osgb2glb(const char* in, const char* out);
