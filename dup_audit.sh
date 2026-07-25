#!/usr/bin/env bash
set -euo pipefail

echo "============================================================"
echo "          AMUNCHAIN DEPENDENCY DUPLICATE AUDIT"
echo "============================================================"

TMP=$(mktemp)

cargo tree --duplicates | tee "$TMP"

echo
echo "============================================================"
echo "                     ANALYSIS"
echo "============================================================"

echo
echo "Major duplicate crates:"
echo "------------------------------------------------------------"

grep -oE '^[a-zA-Z0-9_-]+ v[0-9]+\.[0-9]+\.[0-9]+' "$TMP" \
| awk '
{
    split($2,v,".");
    major[$1]=major[$1]" "v[1];
    vers[$1]=vers[$1]" "$2;
}
END{
    for(c in major){
        n=split(major[c],a," ");
        delete seen
        cnt=0
        for(i=1;i<=n;i++){
            if(a[i]!="" && !(a[i] in seen)){
                seen[a[i]]=1
                cnt++
            }
        }
        if(cnt>1){
            printf("%-25s %s\n",c,vers[c]);
        }
    }
}' | sort

echo
echo "Minor duplicate crates:"
echo "------------------------------------------------------------"

grep -oE '^[a-zA-Z0-9_-]+ v[0-9]+\.[0-9]+\.[0-9]+' "$TMP" \
| awk '
{
    split($2,v,".");
    key=$1;
    major[key]=v[1];
    versions[key]=versions[key]" "$2;
}
END{
    for(c in versions){
        n=split(versions[c],a," ");
        delete seen
        cnt=0
        for(i=1;i<=n;i++){
            if(a[i]!="" && !(a[i] in seen)){
                seen[a[i]]=1
                cnt++
            }
        }
        if(cnt>1){
            print c
        }
    }
}' | sort

echo
echo "============================================================"
echo "                 RISK ASSESSMENT"
echo "============================================================"

critical=0
medium=0
low=0

has_thiserror=$(grep -c "^thiserror v" "$TMP" || true)
has_rand=$(grep -c "^rand v" "$TMP" || true)
has_tokio=$(grep -c "^tokio v" "$TMP" || true)
has_serde=$(grep -c "^serde v" "$TMP" || true)
has_syn=$(grep -c "^syn v" "$TMP" || true)
has_proc=$(grep -c "^proc-macro2 v" "$TMP" || true)
has_quote=$(grep -c "^quote v" "$TMP" || true)
has_heapless=$(grep -c "^heapless v" "$TMP" || true)
has_cpufeatures=$(grep -c "^cpufeatures v" "$TMP" || true)

echo

if [ "$has_thiserror" -gt 1 ]; then
    echo "[MEDIUM ] thiserror       : Multiple major versions detected."
    medium=$((medium+1))
fi

if [ "$has_rand" -gt 1 ]; then
    echo "[MEDIUM ] rand            : Multiple versions detected."
    medium=$((medium+1))
fi

if [ "$has_tokio" -gt 1 ]; then
    echo "[MEDIUM ] tokio           : Multiple versions detected."
    medium=$((medium+1))
fi

if [ "$has_serde" -gt 1 ]; then
    echo "[MEDIUM ] serde           : Multiple versions detected."
    medium=$((medium+1))
fi

if [ "$has_heapless" -gt 1 ]; then
    echo "[LOW    ] heapless        : Duplicate dependency."
    low=$((low+1))
fi

if [ "$has_cpufeatures" -gt 1 ]; then
    echo "[LOW    ] cpufeatures     : Duplicate dependency."
    low=$((low+1))
fi

if [ "$has_syn" -gt 1 ]; then
    echo "[INFO   ] syn             : Normal proc-macro duplication."
fi

if [ "$has_proc" -gt 1 ]; then
    echo "[INFO   ] proc-macro2     : Normal proc-macro duplication."
fi

if [ "$has_quote" -gt 1 ]; then
    echo "[INFO   ] quote           : Normal proc-macro duplication."
fi

echo
echo "============================================================"
echo "                     SCORE"
echo "============================================================"

echo "Critical : $critical"
echo "Medium   : $medium"
echo "Low      : $low"

echo

score=5

((critical>0)) && score=1
((medium>=4)) && score=2
((medium==3)) && score=3
((medium==2)) && score=4

case $score in
5) rating="★★★★★ Excellent";;
4) rating="★★★★☆ Very Good";;
3) rating="★★★☆☆ Good";;
2) rating="★★☆☆☆ Fair";;
1) rating="★☆☆☆☆ Needs Cleanup";;
esac

echo "Overall dependency health:"
echo "$rating"

echo
echo "============================================================"
echo "                 RECOMMENDATIONS"
echo "============================================================"

echo "1. Prefer a single major version of thiserror."
echo "2. Prefer a single version of rand if practical."
echo "3. Ignore syn / quote / proc-macro2 duplication unless all proc-macro crates can be unified."
echo "4. heapless/hash32/cpufeatures duplication is usually low priority."
echo "5. Run:"
echo "      cargo tree -i thiserror"
echo "      cargo tree -i rand"
echo "      cargo tree -i serde"
echo "      cargo tree -i tokio"
echo
echo "============================================================"

rm -f "$TMP"
