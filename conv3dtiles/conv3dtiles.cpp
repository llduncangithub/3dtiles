
#include <osgb23dtile.h>
#include <string>
int main()
{
	const std::string inpath("D:\\实验数据\\倾斜摄影测量数据\\倾斜摄影数据\\result\\Data\\Tile_+000_+000\\Tile_+000_+000.osgb");
	const std::string outpath("D:\\实验数据\\倾斜摄影测量数据\\倾斜摄影数据\\3dtile\\Tile_+000_+000.b3dm");

	bool br = osgb23dtile(inpath.c_str(), outpath.c_str());
	return 0;
}