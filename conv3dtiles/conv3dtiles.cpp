
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
	int retstrlen = 0;

	void* retstr = osgb23dtile_path(inpath.c_str(), outpath.c_str(), box, &retstrlen, 100);

	//����������
	std::string retstr_copy((char*)retstr);

	//�ͷ��˶�̬����malloc���ַ������;
	//������ȷ����̬���뱾exe��crt��MD,����̬���ӵ�,�����Ǿ�̬����(/MT)��;
	//MT��̬����crt�ᵼ��ÿ��ģ�鶼�ж�����heap,��ͬ��ָ���ڲ�ͬ��heap�е��¶ѳ�ͻ,�Ӷ���crash��;
	free(retstr);
#endif

	return 0;
}