#pragma once

extern "C" void* osgb23dtile_path(
	const char* in_path, const char* out_path,
	double *box, int* len, int max_lvl);

extern "C" bool osgb23dtile(
	const char* in, const char* out);

extern "C" bool osgb2glb(const char* in, const char* out);
