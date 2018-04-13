
#include <osgb23dtile.h>
#include <string>
int main()
{
#if 0
	const std::string inpath("D:\\实验数据\\倾斜摄影测量数据\\倾斜摄影数据\\result\\Data\\Tile_+000_+000\\Tile_+000_+000.osgb");
	const std::string outpath("D:\\实验数据\\倾斜摄影测量数据\\倾斜摄影数据\\3dtile\\Tile_+000_+000.b3dm");

	bool br = osgb23dtile(inpath.c_str(), outpath.c_str());
#else
	const std::string inpath("D:\\实验数据\\倾斜摄影测量数据\\倾斜摄影数据\\result\\Data\\Tile_+000_+000\\Tile_+000_+000.osgb");
	const std::string outpath("D:\\实验数据\\倾斜摄影测量数据\\倾斜摄影数据\\3dtile");

	double box[6] = {0.0};
	int retstrlen = 0;

	void* retstr = osgb23dtile_path(inpath.c_str(), outpath.c_str(), box, &retstrlen, 100);

	//拷贝构造了
	std::string retstr_copy((char*)retstr);

	//释放了动态库中malloc的字符串虚存;
	//但必须确保动态库与本exe的crt是MD,即动态链接的,不能是静态链接(/MT)的;
	//MT静态链接crt会导致每个模块都有独立的heap,相同的指针在不同的heap中导致堆冲突,从而会crash掉;
	free(retstr);
#endif

	return 0;
}