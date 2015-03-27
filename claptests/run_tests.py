#!/usr/bin/env python
import sys
import subprocess

failed = False

_bin = './target/release/claptests'
cmds = {'help short:   ': ['{} -h | wc -l'.format(_bin), ['21']],
		'help long:    ': ['{} --help | wc -l'.format(_bin), ['21']],
		'help subcmd:  ': ['{} help | wc -l'.format(_bin), ['21']],
		'flag short:   ': ['{} -f'.format(_bin), ['flag present 1 times',
												  'option NOT present',
												  'positional NOT present', 
												 'subcmd NOT present']],
		'flag long:    ': ['{} --flag'.format(_bin), ['flag present 1 times',
													  'option NOT present', 
													  'positional NOT present', 
													  'subcmd NOT present']],
		'positional:   ': ['{} some'.format(_bin), ['flag NOT present',
												    'option NOT present',
												    'positional present with value: some',
												    'subcmd NOT present']],
		'option short: ': ['{} -o some'.format(_bin), ['flag NOT present',
												       'option present with value: some',
												       'An option: some',
												       'positional NOT present',
												       'subcmd NOT present']],
		'option long:  ': ['{} --option some'.format(_bin), ['flag NOT present',
												             'option present with value: some',
       												         'An option: some',
												             'positional NOT present',
												             'subcmd NOT present']],
		'option long=: ': ['{} --option=some'.format(_bin), ['flag NOT present',
												             'option present with value: some',
       												         'An option: some',
												             'positional NOT present',
												             'subcmd NOT present']]}

def pass_fail(name, check, good):
	global failed
	print(name, end='')
	if len(good) == 1:
		if check == good:
			print('Pass')
			return
		failed = True
		print('Fail')
		return
	_failed = False
	for i, line in enumerate(check):
		if line == good[i]:
			continue
		_failed = True
	if _failed:
		failed = True
		print('Fail')
		return
	print('Pass')


def main():
	for cmd, cmd_v in cmds.items():
		with subprocess.Popen(cmd_v[0], shell=True, stdout=subprocess.PIPE, universal_newlines=True) as proc:
			out = proc.communicate()[0].strip()
			out = out.split('\n')
			pass_fail(cmd, out, cmd_v[1])
	if failed:
		print('One or more tests failed')
		return 1
	print('All tests passed!')

if __name__ == '__main__':
	sys.exit(main())