#include <gmock/gmock.h>

#include <list>
#include <string>
#include <vector>

#include "src/gc/copier_meta.h"

namespace pos
{
class MockCopierMeta : public CopierMeta
{
public:
    using CopierMeta::CopierMeta;
};

} // namespace pos