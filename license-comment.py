import os
import sys


text = "\
//  QQML or the Quiz Question Markup Language.\n\
//  Copyright (C) 2023 'mppbtw'\n\
//\n\
//  This program is free software: you can redistribute it and/or modify\n\
//  it under the terms of the GNU General Public License as published by\n\
//  the Free Software Foundation, either version 3 of the License, or\n\
//  (at your option) any later version.\n\
//\n\
//  This program is distributed in the hope that it will be useful,\n\
//  but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the\n\
//  GNU General Public License for more details.\n\
//\n\
//  You should have received a copy of the GNU General Public License\n\
//  along with this program. If not, see <https://www.gnu.org/licenses/>.\n\n\
"


def is_empty(string: str) -> bool:
    for ch in string:
        if ch not in ["\t", "\n", " ", "\r"]:
            return False
    return True


def remove_comment_lines(lines: list[str]) -> list[str]:
    for i in range(len(lines)):
        if not lines[i].startswith("//"):
            if not is_empty(lines[i]):
                return (lines[i:len(lines)])
    return []


def search_down(path):
    for s in [".gitignore", ".git", "target"]:
        if s in path:
            return
    if os.path.isfile(path):
        if path.endswith(".rs") or path.endswith(".go") or path.endswith(".c"):
            print(path)
            f = open(path, "r+")
            lines = remove_comment_lines(f.readlines())
            f.seek(0)
            f.write(text)
            for line in lines:
                f.write(line)
    else:
        for child in os.listdir(path):
            search_down(path + "/" + child)


if len(sys.argv) == 1:
    search_down(".")
else:
    search_down(sys.argv[1])
