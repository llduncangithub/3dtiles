
#include <osgb23dtile.h>
#include <string>
int main()
{
#if 0
	const std::string inpath("D:\\ʵ������\\��б��Ӱ��������\\��б��Ӱ����\\result\\Data\\Tile_+000_+000\\Tile_+000_+000.osgb");
	const std::string outpath("D:\\ʵ������\\��б��Ӱ��������\\��б��Ӱ����\\3dtile\\Tile_+000_+000.b3dm");

	bool br = osgb23dtile(inpath.c_str(), outpath.c_str());
#else
	const std::string inpath("D:\\ʵ������\\��б��Ӱ��������\\��б��Ӱ����\\result\\Data\\Tile_+000_+000\\Tile_+000_+000.osgb");
	const std::string outpath("D:\\ʵ������\\��б��Ӱ��������\\��б��Ӱ����\\3dtile");

	double box[6] = {0.0};
	bool hr = osgb23dtile_path(inpath.c_str(), outpath.c_str(), box, 100);
#endif

	return 0;
}