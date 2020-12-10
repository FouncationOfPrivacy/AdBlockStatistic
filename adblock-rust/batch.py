import os
import argparse
import json


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rule')
    parser.add_argument('--url')
    parser.add_argument('--result')

    args = parser.parse_args()
    if not args.rule or not args.url or not args.result:
        parser.print_help()
        return

    for file in os.listdir(args.rule):
        if file == '.' or file == '..' or file.endswith('.json'):
            continue
        path_rule = os.path.join(args.rule, file)
        path_txt = os.path.join(args.result, file)
        
        os.system(f'cargo run {path_rule} {args.url} {path_txt}')

        with open(path_txt, 'r') as hdle:
            lines = hdle.readlines()
            count = len(lines)
            path_json = os.path.join(args.result, f'{file}_{count}.json')
            
            summary = {}
            for line in lines:
                split_line = line.split('\t')
                summary[split_line[0]] = int(split_line[1])

            with open(path_json, 'w') as hdle:
                json.dump(summary, hdle)


if __name__ == "__main__":
    main()

