#include <lib.h>

#include <gtest/gtest.h>

TEST(cpptest, consume_vector_from_rust) {
    std::vector<int> vecExpected = {1, 2, 3, 4, 5};
    std::vector<int> vec = get_vector_from_rust(vecExpected.size());
    EXPECT_EQ(vec, vecExpected);
}

// This allows tests to added without an explicit "add_test" call
int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}