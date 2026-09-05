#if UNITY_EDITOR && UNITY_IOS
using UnityEditor;
using UnityEditor.Callbacks;
using UnityEditor.iOS.Xcode;

namespace Wry.Unity.Editor
{
    internal static class WryIosPostBuild
    {
        [PostProcessBuild(850)]
        private static void AddWebKitFramework(BuildTarget target, string path)
        {
            if (target != BuildTarget.iOS) return;
            var projectPath = PBXProject.GetPBXProjectPath(path);
            var project = new PBXProject();
            project.ReadFromFile(projectPath);
            var targetGuid = project.GetUnityMainTargetGuid();
            project.AddFrameworkToProject(targetGuid, "WebKit.framework", false);
            project.WriteToFile(projectPath);
        }
    }
}
#endif
