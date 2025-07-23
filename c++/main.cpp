#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>

int main() {
  using std::ifstream, std::set, std::string,
      std::copy, std::istream_iterator,
      std::inserter, std::multiset, std::cin,
      std::set_difference,
      std::ostream_iterator, std::cout;

  ifstream dictionary("dictionary.txt");
  set<string> dict;
  copy(istream_iterator<string>(dictionary),
       istream_iterator<string>(),
       inserter(dict, dict.begin()));

//   copy(dict.begin(), dict.end(), ostream_iterator<string>(cout, " d \n"));

  // First try
  //set_difference(
  //    istream_iterator<string>(cin),
  //    istream_iterator<string>(),
  //    dict.begin(), dict.end(),
  //    ostream_iterator<string>(cout, "\n"));

  // FIX: change multiset to set.

  // multiset<string> words;
  set<string> words;
  copy(istream_iterator<string>(cin),
       istream_iterator<string>(),
       inserter(words, words.begin()));

//   copy(words.begin(), words.end(), ostream_iterator<string>(cout, " w \n"));

  set_difference(
      words.begin(), words.end(),
      dict.begin(), dict.end(),
      ostream_iterator<string>(cout, "\n"));
}
