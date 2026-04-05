import os
import subprocess as sbp 


result=os.system("git status")
print(result)

choice=input("Do you wish to continue to commit and push changes to git repository (Y/N): ")

if choice[0].lower()=="y":
	os.system("git add --all")
	commit_msg=input("COMMIT MESSAGE: ")
	os.system(f'git commit -m "{commit_msg}"')
	print("Pushing to git repo")
	os.system("git push origin main")
else:
	print("EXITING...")
	exit()