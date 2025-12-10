#!/usr/bin/python

import microbiorust
result = microbiorust.gbk_to_faa_count("Rhiz3841.gbk.gb")
outfile = open("rhiz.txt", 'w')
outfile.write("{}\n".format(result))
